# 使用Rust构建简单的文件传输系统

### 消息结构：每个消息包括：

- 文件名长度（1字节）
- 文件名（可变长度）
- 文件大小（8字节，小端序）
- 文件数据（可变长度）
##### 编码：在通过网络发送之前，将FileMessage结构编码为字节数组。

##### 解码：接收到的字节数组被解码回FileMessage结构。