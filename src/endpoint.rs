pub trait Outcome {}

pub trait Model<D, O, E>
where
    D: EndpointDbConnection,
    O: Outcome,
    E: EndpointError,
{
    fn model(&self, db_connection: &D, request_body: &String) -> Result<O, E>;
}

pub trait Endpoint {}

pub trait EndpointError {}

pub trait EndpointDbConnection {}

pub trait EndpointResult {}

pub trait Name {
    fn name(&self) -> &'static str;
}

pub trait View<O, E, R>
where
    O: Outcome,
    E: EndpointError,
    R: EndpointResult,
{
    fn view(&self, result: Result<O, E>) -> R;
}

pub trait Presenter<T, D, O, E, R>
where
    T: Endpoint + Name + Model<D, O, E> + View<O, E, R>,
    D: EndpointDbConnection,
    O: Outcome,
    E: EndpointError,
    R: EndpointResult,
{
    fn presenter(endpoint: T, db_connection: &D, request_body: &String) -> R {
        let endpoint_result = endpoint.view(endpoint.model(db_connection, request_body));
        endpoint_result
    }
}

