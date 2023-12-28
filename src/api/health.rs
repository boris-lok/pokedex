use rouille;

pub fn serve() -> rouille::Response {
    rouille::Response::text("Gotta catch them all!")
}