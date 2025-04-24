use std::collections::HashMap;

use crate::yang::*;

/// Resolves references between YANG nodes.
pub struct ReferenceResolver {
    reference_nodes: ReferenceNodes,
    imported_modules: HashMap<String, ReferenceNodes>,
    prefix_to_module: HashMap<String, String>,
}

impl ReferenceResolver {
    /// Create a new reference resolver with the given reference information
    pub fn new(
        reference_nodes: ReferenceNodes,
        imported_modules: HashMap<String, ReferenceNodes>,
        prefix_to_module: HashMap<String, String>,
    ) -> Self {
        Self {
            reference_nodes,
            imported_modules,
            prefix_to_module,
        }
    }

    /// Start resolving references by walking the tree. Walks only through nodes that can actually have references.
    pub fn resolve_references(&self, module: &mut Module) {
        for node in &mut module.body {
            self.resolve_schema_node_references(node, "/");
        }
    }

    fn resolve_schema_node_references(&self, node: &mut SchemaNode, path: &str) {
        match node {
            SchemaNode::DataDef(data_def) => self.resolve_data_def_references(data_def, path),
            SchemaNode::Rpc(rpc) => self.resolve_rpc_references(rpc, path),
            SchemaNode::Notification(notification) => self.resolve_notification_references(notification, path),
        }
    }

    fn resolve_data_def_references(&self, data_def: &mut DataDef, path: &str) {
        match data_def {
            DataDef::Container(container) => {
                let container_path = format!("{}{}/", path, container.name);
                self.resolve_container_references(container, &container_path);
            }
            DataDef::List(list) => {
                let list_path = format!("{}{}/", path, list.name);
                self.resolve_list_references(list, &list_path);
            }
            DataDef::Choice(choice) => {
                let choice_path = format!("{}{}/", path, choice.name);
                self.resolve_choice_references(choice, &choice_path);
            }
            _ => {}
        }
    }

    fn resolve_container_references(&self, container: &mut Container, path: &str) {
        self.resolve_data_defs(&mut container.data_defs, path);

        for action in &mut container.actions {
            let action_path = format!("{}{}/", path, action.name);
            self.resolve_action_references(action, &action_path);
        }

        for notification in &mut container.notifications {
            let notification_path = format!("{}{}/", path, notification.name);
            self.resolve_notification_references(notification, &notification_path);
        }
    }

    fn resolve_list_references(&self, list: &mut List, path: &str) {
        self.resolve_data_defs(&mut list.data_defs, path);

        for action in &mut list.actions {
            let action_path = format!("{}{}/", path, action.name);
            self.resolve_action_references(action, &action_path);
        }

        for notification in &mut list.notifications {
            let notification_path = format!("{}{}/", path, notification.name);
            self.resolve_notification_references(notification, &notification_path);
        }
    }

    fn resolve_choice_references(&self, choice: &mut Choice, path: &str) {
        for case in &mut choice.cases {
            match case {
                Case::LongCase(long_case) => {
                    let case_path = format!("{}{}/", path, long_case.name);
                    self.resolve_long_case_references(long_case, &case_path);
                }
                Case::ShortCase(short_case) => self.resolve_short_case_references(short_case, path),
            }
        }
    }

    fn resolve_long_case_references(&self, long_case: &mut LongCase, path: &str) {
        self.resolve_data_defs(&mut long_case.data_defs, path);
    }

    fn resolve_short_case_references(&self, short_case: &mut ShortCase, path: &str) {
        match short_case {
            ShortCase::Container(container) => {
                let container_path = format!("{}{}/", path, container.name);
                self.resolve_container_references(container, &container_path);
            }
            ShortCase::List(list) => {
                let list_path = format!("{}{}/", path, list.name);
                self.resolve_list_references(list, &list_path);
            }
            ShortCase::Choice(choice) => {
                let choice_path = format!("{}{}/", path, choice.name);
                self.resolve_choice_references(choice, &choice_path);
            }
            _ => {}
        }
    }

    fn resolve_augment_references(&self, augment: &mut Augment, path: &str) {
        self.resolve_data_defs(&mut augment.data_defs, path);

        for case in &mut augment.cases {
            match case {
                Case::LongCase(long_case) => {
                    let case_path = format!("{}{}/", path, long_case.name);
                    self.resolve_long_case_references(long_case, &case_path);
                }
                Case::ShortCase(short_case) => self.resolve_short_case_references(short_case, path),
            }
        }

        for action in &mut augment.actions {
            let action_path = format!("{}{}/", path, action.name);
            self.resolve_action_references(action, &action_path);
        }

        for notification in &mut augment.notifications {
            let notification_path = format!("{}{}/", path, notification.name);
            self.resolve_notification_references(notification, &notification_path);
        }
    }

    fn resolve_action_references(&self, action: &mut Action, path: &str) {
        if let Some(input) = &mut action.input {
            let input_path = format!("{}input/", path);
            self.resolve_data_defs(&mut input.data_defs, &input_path);
        }

        if let Some(output) = &mut action.output {
            let output_path = format!("{}output/", path);
            self.resolve_data_defs(&mut output.data_defs, &output_path);
        }
    }

    fn resolve_rpc_references(&self, rpc: &mut Rpc, path: &str) {
        if let Some(input) = &mut rpc.input {
            let input_path = format!("{}input/", path);
            self.resolve_data_defs(&mut input.data_defs, &input_path);
        }

        if let Some(output) = &mut rpc.output {
            let output_path = format!("{}output/", path);
            self.resolve_data_defs(&mut output.data_defs, &output_path);
        }
    }

    fn resolve_notification_references(&self, notification: &mut Notification, path: &str) {
        self.resolve_data_defs(&mut notification.data_defs, path);
    }

    /// Find a grouping by traversing from current path up to the root or from imported modules
    /// when a prefix is present.
    fn find_grouping(&self, grouping_name: &str, current_path: &str) -> Option<&Grouping> {
        // Check if the grouping name has a prefix (indicating an imported module).
        if let Some(idx) = grouping_name.find(':') {
            let prefix = &grouping_name[..idx];
            let name = &grouping_name[idx + 1..];

            // Look up the module name from the prefix.
            if let Some(module_name) = self.prefix_to_module.get(prefix) {
                // Look up the imported module's reference nodes.
                if let Some(ref_nodes) = self.imported_modules.get(module_name) {
                    // Look for the grouping in the imported module's reference nodes.
                    // Imported groupings are expected to be at the top level.
                    let path = format!("/{}", name);

                    #[cfg(debug_assertions)]
                    println!(
                        "Looking for imported grouping {} in module {} (path: {})",
                        name, module_name, path
                    );

                    if let Some(grouping) = ref_nodes.groupings.get(&path) {
                        #[cfg(debug_assertions)]
                        println!("Found imported grouping {} in module {}", name, module_name);
                        return Some(grouping);
                    }
                }

                #[cfg(debug_assertions)]
                println!("Failed to find imported grouping {} in module {}", name, module_name);
            }

            // If prefix resolution failed, return None.
            return None;
        }

        // Non-prefixed grouping: look in local module using hierarchical resolution.
        // Start from the current path and work our way up.
        let mut search_path = current_path.to_string();

        loop {
            // Try to find the grouping in the current search path.
            let full_path = format!("{}{}", search_path, grouping_name);

            #[cfg(debug_assertions)]
            println!("Looking for local grouping {} at path {}", grouping_name, full_path);

            if let Some(grouping) = self.reference_nodes.groupings.get(&full_path) {
                #[cfg(debug_assertions)]
                println!("Found local grouping {} at path {}", grouping_name, full_path);
                return Some(grouping);
            }

            // If we're at the root, we've exhausted all options.
            if search_path == "/" {
                #[cfg(debug_assertions)]
                println!("Exhausted all options for local grouping {}", grouping_name);
                break;
            }

            // Move up one level - remove the last directory segment.
            let segments: Vec<&str> = search_path.split('/').collect();
            if segments.len() <= 2 {
                // We're already at the root level, try root itself.
                search_path = "/".to_string();
            } else {
                // Remove last directory and construct new path.
                search_path = segments[..segments.len() - 2].join("/");
                if !search_path.ends_with('/') {
                    search_path.push('/');
                }
            }
        }

        None
    }

    /// The core method that resolves all references in a vector of DataDef nodes.
    fn resolve_data_defs(&self, data_defs: &mut Vec<DataDef>, path: &str) {
        // Find indices of all Uses nodes.
        let mut uses_indices: Vec<(usize, String)> = Vec::new();

        // Collect all Uses nodes and their grouping names.
        for (idx, data_def) in data_defs.iter().enumerate() {
            if let DataDef::Uses(uses) = data_def {
                uses_indices.push((idx, uses.grouping.clone()));
            }
        }

        // Process Uses nodes in reverse order to avoid index invalidation.
        for (idx, grouping_name) in uses_indices.iter().rev() {
            // Look up the grouping by hierarchical path resolution.
            if let Some(grouping) = self.find_grouping(&grouping_name, path) {
                // Clone the data_defs from the grouping.
                let grouping_data_defs = grouping.data_defs.clone();
                let data_defs_len = grouping_data_defs.len();

                // Remove the Uses node as it is not needed in the final data tree.
                data_defs.remove(*idx);

                // Insert all data_defs from the grouping at the same position.
                for (inner_idx, data_def) in grouping_data_defs.into_iter().enumerate() {
                    data_defs.insert(*idx + inner_idx, data_def);
                }

                // Process the newly inserted nodes to resolve any nested references.
                for inner_idx in 0..data_defs_len {
                    if let Some(data_def) = data_defs.get_mut(*idx + inner_idx) {
                        self.resolve_data_def_references(data_def, path);
                    }
                }
            }
        }

        // Recursively resolve any references in remaining nodes
        for data_def in data_defs.iter_mut() {
            self.resolve_data_def_references(data_def, path);
        }
    }
}
