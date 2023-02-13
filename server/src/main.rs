use cargo::resp_parser;
mod server;
fn main(){
    server::launch_server(6379)
}
