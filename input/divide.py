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

# 机器码形式的 shellcode，把自己的 shellcode 输入到这里（机器码形式）
shellcode = b""

c_array = ""
for byte in shellcode:
    c_array += f"0x{byte:02x},"
c_array = c_array.rstrip(",")

outpath = "output.txt"
extract_and_write(outpath, c_array)