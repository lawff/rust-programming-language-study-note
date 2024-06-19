use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 可以防止积压
fn main() {
    // 创建一个同步通道，指定缓冲区大小为 10
    let (tx, rx) = mpsc::sync_channel(10);

    // 生产者线程
    let producer = thread::spawn(move || {
        for i in 0..100 {
            println!("生产者: 发送 {}", i);
            tx.send(i).unwrap(); // 如果通道已满，这里会阻塞
            thread::sleep(Duration::from_millis(50)); // 模拟文件读取耗时
        }
    });

    // 消费者线程
    let consumer = thread::spawn(move || {
        for received in rx {
            println!("消费者: 接收 {}", received);
            thread::sleep(Duration::from_millis(1000)); // 模拟数据处理耗时
        }
    });

    // 等待两个线程结束
    producer.join().unwrap();
    consumer.join().unwrap();
}
