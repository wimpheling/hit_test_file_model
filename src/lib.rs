use hit::field_types::*;
use hit::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;
use std::rc::Rc;

pub struct ModelModelKernel {
    models: HashMap<String, Rc<Model>>,
    pub model_types_index: Rc<RefCell<ModelTypeIndexer>>,
}
impl ModelModelKernel {
    fn new(models: HashMap<String, Rc<Model>>) -> Self {
        ModelModelKernel {
            models: models,
            model_types_index: Rc::new(RefCell::new(ModelTypeIndexer::new())),
        }
    }
}
impl Kernel<Rc<Model>, HitEntry> for ModelModelKernel {
    fn get_model(&self, name: &str) -> Result<Rc<Model>, HitError> {
        match self.models.get(name) {
            Some(model) => Ok(model.clone()),
            None => Err(HitError::ModelDoesNotExist(String::from(name))),
        }
    }
    fn get_instantiable_models(&self) -> Vec<&Model> {
        match self.models.get("file/project") {
            Some(model) => vec![&model],
            None => vec![],
        }
    }
    fn get_plugins(&self) -> Plugins<Rc<Model>, HitEntry> {
        let mut plugins = Plugins::new();
        // TODO : this should be built in HIT ?
        plugins.delete_plugins.push(self.model_types_index.clone());
        plugins.init_plugins.push(self.model_types_index.clone());
        plugins.plugins.push(self.model_types_index.clone());
        plugins
    }
    fn get_models(&self) -> Vec<String> {
        let mut output = vec![];
        for (key, _) in self.models.iter() {
            output.push(key.to_string());
        }
        output
    }
}

pub fn create_kernel() -> ModelModelKernel {
    let project = modele!("file/filesystem", "Filesystem" =>
        "name": FieldTypeString {
            required: true
        },
        "folders": FieldTypeSubobjectArray {
            authorized_models: vec!["file/folder".to_string()]
        },
        "items": FieldTypeSubobjectArray {
            authorized_models: vec!["file/file".to_string()]
        }
    );

    let folder = modele!("file/folder", "Folder" =>
        "name": FieldTypeString {
            required: true
        },
        "items": FieldTypeSubobjectArray {
            authorized_models: vec![String::from("file/file")]
        },
        "folders": FieldTypeSubobjectArray {
            authorized_models: vec![String::from("file/folder")]
        },
    );

    let file = modele!("file/file", "Model" =>
        "name": FieldTypeString {
            required: true
        },
    );

    let mut models = HashMap::new();
    models.insert(String::from("file/project"), project);
    models.insert(String::from("file/folder"), folder);
    models.insert(String::from("file/file"), file);
    return ModelModelKernel::new(models);
}
