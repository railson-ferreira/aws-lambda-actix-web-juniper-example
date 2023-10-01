use juniper::{RootNode, EmptyMutation, EmptySubscription};

pub struct QueryRoot;

struct HelloWorld {
    message: String,
}

#[juniper::graphql_object(description = "There is a message here")]
impl HelloWorld{
    pub fn message(&self)->String{
        self.message.clone()
    }
}



#[juniper::graphql_object]
impl QueryRoot{
    fn hello(name: Option<String>) -> HelloWorld {
        if name.is_none() {
            return HelloWorld {
                message: "Hello World!".to_string()
            };
        }
        HelloWorld {
            message: format!("Hello {}!", name.unwrap().to_string())
        }
    }
}


// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    return Schema::new(QueryRoot{}, EmptyMutation::new(), EmptySubscription::new());
}
