use crate::ast::*;
use std::collections::HashMap;

/// Resolver for YANG references like Uses nodes
pub struct ReferenceResolver {
    groupings: HashMap<String, Grouping>,
}

impl ReferenceResolver {
    pub fn new(groupings: HashMap<String, Grouping>) -> Self {
        Self { groupings }
    }

    /// Resolve all references in a YANG file
    pub fn resolve_references(&self, file: &mut YangFile) {
        match file {
            YangFile::Module(module) => self.resolve_module_references(module, "/"),
            YangFile::Submodule(submodule) => self.resolve_submodule_references(submodule, "/"),
        }
    }

    /// Resolve references in a module
    fn resolve_module_references(&self, module: &mut Module, path: &str) {
        for node in &mut module.body {
            self.resolve_schema_node_references(node, path);
        }
    }

    /// Resolve references in a submodule
    fn resolve_submodule_references(&self, submodule: &mut Submodule, path: &str) {
        // Process each schema node in the submodule body
        for node in &mut submodule.body {
            self.resolve_schema_node_references(node, path);
        }
    }

    /// Resolve references in a schema node
    fn resolve_schema_node_references(&self, node: &mut SchemaNode, path: &str) {
        match node {
            SchemaNode::DataDef(data_def) => self.resolve_data_def_references(data_def, path),
            SchemaNode::Augment(augment) => self.resolve_augment_references(augment, path),
            SchemaNode::Rpc(rpc) => self.resolve_rpc_references(rpc, path),
            SchemaNode::Notification(notification) => self.resolve_notification_references(notification, path),
            _ => {}
        }
    }

    /// Resolve references in a data definition node
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

    /// Resolve references in a container
    fn resolve_container_references(&self, container: &mut Container, path: &str) {
        // Process data definitions, which may contain Uses references
        self.resolve_data_defs_vec(&mut container.data_defs, path);

        // Process actions
        for action in &mut container.actions {
            let action_path = format!("{}{}/", path, action.name);
            self.resolve_action_references(action, &action_path);
        }

        // Process notifications
        for notification in &mut container.notifications {
            let notification_path = format!("{}{}/", path, notification.name);
            self.resolve_notification_references(notification, &notification_path);
        }
    }

    /// Resolve references in a list
    fn resolve_list_references(&self, list: &mut List, path: &str) {
        // Process data definitions
        self.resolve_data_defs_vec(&mut list.data_defs, path);

        // Process actions
        for action in &mut list.actions {
            let action_path = format!("{}{}/", path, action.name);
            self.resolve_action_references(action, &action_path);
        }

        // Process notifications
        for notification in &mut list.notifications {
            let notification_path = format!("{}{}/", path, notification.name);
            self.resolve_notification_references(notification, &notification_path);
        }
    }

    /// Resolve references in a choice
    fn resolve_choice_references(&self, choice: &mut Choice, path: &str) {
        // Process each case in the choice
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

    /// Resolve references in a long case
    fn resolve_long_case_references(&self, long_case: &mut LongCase, path: &str) {
        // Process data definitions
        self.resolve_data_defs_vec(&mut long_case.data_defs, path);
    }

    /// Resolve references in a short case
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
            // Leaf, LeafList, AnyData, and Anyxml don't contain Uses references
            _ => {}
        }
    }

    /// Resolve references in an augment
    fn resolve_augment_references(&self, augment: &mut Augment, path: &str) {
        // Process data definitions
        self.resolve_data_defs_vec(&mut augment.data_defs, path);

        // Process cases
        for case in &mut augment.cases {
            match case {
                Case::LongCase(long_case) => {
                    let case_path = format!("{}{}/", path, long_case.name);
                    self.resolve_long_case_references(long_case, &case_path);
                }
                Case::ShortCase(short_case) => self.resolve_short_case_references(short_case, path),
            }
        }

        // Process actions
        for action in &mut augment.actions {
            let action_path = format!("{}{}/", path, action.name);
            self.resolve_action_references(action, &action_path);
        }

        // Process notifications
        for notification in &mut augment.notifications {
            let notification_path = format!("{}{}/", path, notification.name);
            self.resolve_notification_references(notification, &notification_path);
        }
    }

    /// Resolve references in an action
    fn resolve_action_references(&self, action: &mut Action, path: &str) {
        if let Some(input) = &mut action.input {
            let input_path = format!("{}input/", path);
            self.resolve_data_defs_vec(&mut input.data_defs, &input_path);
        }

        if let Some(output) = &mut action.output {
            let output_path = format!("{}output/", path);
            self.resolve_data_defs_vec(&mut output.data_defs, &output_path);
        }
    }

    /// Resolve references in an RPC
    fn resolve_rpc_references(&self, rpc: &mut Rpc, path: &str) {
        if let Some(input) = &mut rpc.input {
            let input_path = format!("{}input/", path);
            self.resolve_data_defs_vec(&mut input.data_defs, &input_path);
        }

        if let Some(output) = &mut rpc.output {
            let output_path = format!("{}output/", path);
            self.resolve_data_defs_vec(&mut output.data_defs, &output_path);
        }
    }

    /// Resolve references in a notification
    fn resolve_notification_references(&self, notification: &mut Notification, path: &str) {
        self.resolve_data_defs_vec(&mut notification.data_defs, path);
    }

    /// Find a grouping by traversing from current path up to the root
    fn find_grouping(&self, grouping_name: &str, current_path: &str) -> Option<&Grouping> {
        // Start from the current path and work our way up
        let mut search_path = current_path.to_string();

        loop {
            // Try to find the grouping in the current search path
            let full_path = format!("{}{}", search_path, grouping_name);

            println!("Looking for grouping {} at path {}", grouping_name, full_path);

            if let Some(grouping) = self.groupings.get(&full_path) {
                println!("Found grouping {} at path {}", grouping_name, full_path);
                return Some(grouping);
            }

            // If we're at the root, we've exhausted all options
            if search_path == "/" {
                break;
            }

            // Move up one level - remove the last directory segment
            let segments: Vec<&str> = search_path.split('/').collect();
            if segments.len() <= 2 {
                // We're already at the root level, try root itself
                search_path = "/".to_string();
            } else {
                // Remove last directory and construct new path
                search_path = segments[..segments.len() - 2].join("/");
                if !search_path.ends_with('/') {
                    search_path.push('/');
                }
            }

            println!("Moved up to path {}", search_path);
        }

        println!("Grouping {} not found", grouping_name);
        None
    }

    /// The core method that resolves all Uses references in a vector of DataDef nodes
    fn resolve_data_defs_vec(&self, data_defs: &mut Vec<DataDef>, path: &str) {
        // Find indices of all Uses nodes
        let mut uses_indices: Vec<(usize, String)> = Vec::new();

        // First pass: collect all Uses nodes and their grouping names
        for (i, data_def) in data_defs.iter().enumerate() {
            if let DataDef::Uses(uses) = data_def {
                uses_indices.push((i, uses.grouping.clone()));
            }
        }

        // Process Uses nodes in reverse order to avoid index invalidation
        for (idx, grouping_name) in uses_indices.iter().rev() {
            // Look up the grouping by hierarchical path resolution
            if let Some(grouping) = self.find_grouping(&grouping_name, path) {
                // Clone the data_defs from the grouping
                let grouping_data_defs = grouping.data_defs.clone();
                let data_defs_len = grouping_data_defs.len();

                // Remove the Uses node
                data_defs.remove(*idx);

                // Insert all data_defs from the grouping at the same position
                for (j, data_def) in grouping_data_defs.into_iter().enumerate() {
                    data_defs.insert(*idx + j, data_def);
                }

                // Process the newly inserted nodes to resolve any nested references
                for j in 0..data_defs_len {
                    if let Some(data_def) = data_defs.get_mut(*idx + j) {
                        self.resolve_data_def_references(data_def, path);
                    }
                }
            }
        }

        // Second pass: recursively resolve any references in remaining nodes
        for data_def in data_defs.iter_mut() {
            self.resolve_data_def_references(data_def, path);
        }
    }
}
