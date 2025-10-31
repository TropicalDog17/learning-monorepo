use std::{any::{self, Any, TypeId}, collections::HashMap};

type Type = TypeId;

type Impl = Box<dyn Fn() -> Box<dyn Any>>;
struct DIContainer {
    pub registry: HashMap<Type, Impl>
}

impl DIContainer {
    pub fn new() -> Self {
        Self {
            registry: HashMap::default()
        }
    }
    
    pub fn register(&mut self, type_identifier: Type, concrete_impl: Impl)  {
        self.registry.insert(type_identifier, concrete_impl);
    }
    pub fn resolve<T: 'static>(&self) -> Option<Box<T>> {
        self.registry
            .get(&TypeId::of::<T>())
            .map(|f| f())
            .and_then(|boxed_any| boxed_any.downcast::<T>().ok())
    }
}

#[cfg(test)]
mod tests {
    use std::{any::TypeId, cell::RefCell, rc::Rc};

    use crate::DIContainer;


    struct ServiceA {
        value: i32,
    }
    struct ServiceB {
        a: Box<ServiceA>,
    }

    impl ServiceB {
        pub fn print_value(&self) {
            println!("Value of a: {}", self.a.value)
        }
    }
    
    #[test]
    fn happy() {
        let container = Rc::new(RefCell::new(DIContainer::new()));

        container.borrow_mut().register(TypeId::of::<ServiceA>(), Box::new(|| Box::new(ServiceA { value: 42 })));
        let container_clone = Rc::clone(&container);
        container.borrow_mut().register(TypeId::of::<ServiceB>(), Box::new(move || {
            let a = container_clone.borrow().resolve::<ServiceA>().unwrap();
            Box::new(ServiceB { a })
        }));

        container.borrow().resolve::<ServiceB>().unwrap().print_value();
    }
}