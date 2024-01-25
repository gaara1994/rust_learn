use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut count = 0;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        count = count+1;
        println!("连接{}已建立:",count);
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    //创建一个缓冲区
    let mut buffer = [0;512];
    //stream.read会从TcpStream中读取数据并将其存储至缓冲区中
    stream.read(&mut buffer).unwrap();
    //函数String:: from_utf8_lossy可以接收一个&[u8]并产生对应的String。
    println!("请求:\n {}",String::from_utf8_lossy(&buffer[..]));
    /*
 GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/7.68.0
    */

    //将/请求的相关数据硬编码到了变量get中
    //由于缓冲区中接收的数据是原始字节，所以我们使用字节字符串语法b""将get的文本内容转换为字节字符串
    let get = b"GET / HTTP/1.1";

    //检查buffer中的数据是否以get中的字节开头
    if buffer.starts_with(get) {
        //返回一个html文件
        let contents = fs::read_to_string("hello.html").unwrap();
        // println!("contents=={}",contents);
        //格式化字符串
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);
        //由于stream的write方法只接收&[u8]类型值作为参数，所以我们需要调用response的as_bytes方法来将它的字符串转换为字节，并将这些字节发送到连接中去
        stream.write(response.as_bytes()).unwrap();
        //flush调用会等待并阻止程序继续运行直到所有字节都被写入连接中
        stream.flush().unwrap()
    }else {
        // 一些其他的请求
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        //返回一个404.html文件
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}",status_line,contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap()
    }


}
