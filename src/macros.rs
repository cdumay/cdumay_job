/// Public macro to define a task struct.
/// 
/// You can optionally override the types of `metadata` and/or `params`.
/// 
/// 
#[macro_export]
macro_rules! define_task {
    // Case: both metadata and params types are provided
    (
        $name:ident {
            params: $params_ty:ty,
            metadata: $meta_ty:ty $(,)?
        }
    ) => {
        $crate::define_task_impl! {
            $name,
            $params_ty,
            $meta_ty
        }
    };

    // Case: only metadata type is provided (params uses default)
    (
        $name:ident {
            metadata: $meta_ty:ty $(,)?
        }
    ) => {
        $crate::define_task_impl! {
            $name,
            std::collections::BTreeMap<String, serde_value::Value>,
            $meta_ty
        }
    };

    // Case: only params type is provided (metadata uses default)
    (
        $name:ident {
            params: $params_ty:ty $(,)?
        }
    ) => {
        $crate::define_task_impl! {
            $name,
            $params_ty,
            std::collections::BTreeMap<String, serde_value::Value>
        }
    };

    // Case: no types provided, use defaults for both metadata and params
    ($name:ident) => {
        $crate::define_task_impl! {
            $name,
            std::collections::BTreeMap<String, serde_value::Value>,
            std::collections::BTreeMap<String, serde_value::Value>
        }
    };
}

/// Private helper macro to generate the struct and its constructor
#[macro_export]
macro_rules! define_task_impl {
    ($name:ident, $params_ty:ty, $meta_ty:ty) => {
        #[doc = concat!("Task : ", module_path!(), ".", stringify!($name))]
        pub struct $name {
            metadata: $meta_ty,
            params: $params_ty,
            result: cdumay_job::Result,
            status: cdumay_job::Status,
            uuid: uuid::Uuid,
        }
        impl $name {
            pub fn new(params: Option<$params_ty>, metadata: Option<$meta_ty>) -> Self {
                let uuid = uuid::Uuid::new_v4();
                Self {
                    metadata: metadata.unwrap_or_default(),
                    params: params.unwrap_or_default(),
                    result: cdumay_job::ResultBuilder::default().uuid(uuid).build(),
                    status: cdumay_job::Status::default(),
                    uuid: uuid,
                }
            }
            pub fn params(&self) ->  &$params_ty {
                &self.params
            }
            pub fn metadata(&self) ->  &$meta_ty {
                &self.metadata
            }
            pub fn metadata_mut(&mut self) -> &mut $meta_ty {
                &mut self.metadata
            }
        }

        impl cdumay_job::TaskInfo for $name {
            fn path(&self) -> String {
                format!("{}::{}", module_path!(), stringify!($name))
            }
            fn status(&self) -> cdumay_job::Status {
                self.status.clone()
            }
            fn status_mut(&mut self) -> &mut cdumay_job::Status {
                &mut self.status
            }
            fn uuid(&self) -> uuid::Uuid {
                self.uuid
            }
            fn result(&self) -> cdumay_job::Result {
                self.result.clone()
            }
            fn result_mut(&mut self) -> &mut cdumay_job::Result {
                &mut self.result
            }
        }
    };
}
/// Public macro to define a task struct.
///
/// You can optionally override the types of `metadata` and/or `params`.
///
///
#[macro_export]
macro_rules! define_operation {
    // Case: both metadata and params types are provided
    (
        $name:ident {
            params: $params_ty:ty,
            metadata: $meta_ty:ty $(,)?
        }
    ) => {
        $crate::define_operation_impl! {
            $name,
            $params_ty,
            $meta_ty
        }
    };

    // Case: only metadata type is provided (params uses default)
    (
        $name:ident {
            metadata: $meta_ty:ty $(,)?
        }
    ) => {
        $crate::define_operation_impl! {
            $name,
            std::collections::BTreeMap<String, serde_value::Value>,
            $meta_ty
        }
    };

    // Case: only params type is provided (metadata uses default)
    (
        $name:ident {
            params: $params_ty:ty $(,)?
        }
    ) => {
        $crate::define_operation_impl! {
            $name,
            $params_ty,
            std::collections::BTreeMap<String, serde_value::Value>
        }
    };

    // Case: no types provided, use defaults for both metadata and params
    ($name:ident) => {
        $crate::define_operation_impl! {
            $name,
            std::collections::BTreeMap<String, serde_value::Value>,
            std::collections::BTreeMap<String, serde_value::Value>
        }
    };
}

/// Private helper macro to generate the struct and its constructor
#[macro_export]
macro_rules! define_operation_impl {
    ($name:ident, $params_ty:ty, $meta_ty:ty) => {
        #[doc = concat!("Task : ", module_path!(), ".", stringify!($name))]
        pub struct $name {
            metadata: $meta_ty,
            params: $params_ty,
            result: cdumay_job::Result,
            status: cdumay_job::Status,
            uuid: uuid::Uuid,
            tasks: Vec<Box<dyn TaskExec>>,
        }
        impl $name {
            pub fn new(params: Option<$params_ty>, metadata: Option<$meta_ty>) -> Self {
                let uuid = uuid::Uuid::new_v4();
                Self {
                    metadata: metadata.unwrap_or_default(),
                    params: params.unwrap_or_default(),
                    result: cdumay_job::ResultBuilder::default().uuid(uuid).build(),
                    status: cdumay_job::Status::default(),
                    uuid: uuid,
                    tasks: Vec::default(),
                }
            }
            pub fn params(&self) ->  &$params_ty {
                &self.params
            }
            pub fn metadata(&self) ->  &$meta_ty {
                &self.metadata
            }
            pub fn metadata_mut(&mut self) -> &mut $meta_ty {
                &mut self.metadata
            }
        }

        impl cdumay_job::TaskInfo for $name {
            fn path(&self) -> String {
                format!("{}::{}", module_path!(), stringify!($name))
            }
            fn status(&self) -> cdumay_job::Status {
                self.status.clone()
            }
            fn status_mut(&mut self) -> &mut cdumay_job::Status {
                &mut self.status
            }
            fn uuid(&self) -> uuid::Uuid {
                self.uuid
            }
            fn result(&self) -> cdumay_job::Result {
                self.result.clone()
            }
            fn result_mut(&mut self) -> &mut cdumay_job::Result {
                &mut self.result
            }
        }

        impl cdumay_job::OperationInfo for $name {
            fn tasks(&self) -> &Vec<Box<dyn TaskExec>> {
                &self.tasks
            }
            fn tasks_mut(&mut self) -> &mut Vec<Box<dyn TaskExec>> {
                &mut self.tasks
            }
        }
    };
}
