use crate::{CompositeTypeRef, CompositeTypeWeakRef, InternalDataModelRef, ModelRef, ModelWeakRef};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub enum ParentContainer {
    Model(ModelWeakRef),
    CompositeType(CompositeTypeWeakRef),
}

impl ParentContainer {
    pub fn internal_data_model(&self) -> InternalDataModelRef {
        // Unwraps are safe - the models and composites must exist after DML translation.
        match self {
            ParentContainer::Model(model) => model.upgrade().unwrap().internal_data_model(),
            ParentContainer::CompositeType(composite) => composite.upgrade().unwrap().internal_data_model(),
        }
    }

    pub fn as_model(&self) -> Option<ModelRef> {
        match self {
            ParentContainer::Model(m) => m.upgrade(),
            ParentContainer::CompositeType(_) => None,
        }
    }

    pub fn as_model_weak(&self) -> Option<ModelWeakRef> {
        match self {
            ParentContainer::Model(m) => Some(m.clone()),
            ParentContainer::CompositeType(_) => None,
        }
    }

    fn as_composite(&self) -> Option<CompositeTypeRef> {
        match self {
            ParentContainer::Model(_) => None,
            ParentContainer::CompositeType(ct) => ct.upgrade(),
        }
    }
}

impl From<ModelWeakRef> for ParentContainer {
    fn from(model: ModelWeakRef) -> Self {
        Self::Model(model)
    }
}

impl From<CompositeTypeWeakRef> for ParentContainer {
    fn from(composite: CompositeTypeWeakRef) -> Self {
        Self::CompositeType(composite)
    }
}

impl Debug for ParentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParentContainer::Model(m) => f
                .debug_struct("ParentContainer")
                .field("enum_variant", &"Model")
                .field("name", &m.upgrade().unwrap().name)
                .finish(),

            ParentContainer::CompositeType(ct) => f
                .debug_struct("ParentContainer")
                .field("enum_variant", &"CompositeType")
                .field("name", &ct.upgrade().unwrap().name)
                .finish(),
        }
    }
}

impl Hash for ParentContainer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Unwraps are safe - the models and composites must exist after DML translation.
        match self {
            ParentContainer::Model(model) => model.upgrade().unwrap().hash(state),
            ParentContainer::CompositeType(composite) => composite.upgrade().unwrap().hash(state),
        }
    }
}

impl Eq for ParentContainer {}

impl PartialEq for ParentContainer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParentContainer::Model(_), ParentContainer::Model(_)) => self.as_model() == other.as_model(),
            (ParentContainer::CompositeType(_), ParentContainer::CompositeType(_)) => {
                self.as_composite() == other.as_composite()
            }
            _ => false,
        }
    }
}
