import hashlib
import sys

args = sys.argv

def calculate_file(file, chunk_size=4096):
    hash_md5 = hashlib.md5()

    with open(file, 'rb') as f:
        for chunk in iter(lambda: f.read(chunk_size), b''):
            hash_md5.update(chunk)

    return hash_md5.hexdigest()

def main():
    file_1 = args[1]
    file_2 = args[2]

    file_1 = calculate_file(file_1)

    print(f"First Fine 1: {file_1}")

    file_2 = calculate_file(file_2)

    print(f"First Fine 2: {file_2}")

    if file_1 == file_2:
        print("Su integracion esta bien")
    else:
        print("Falta archivos")

if __name__ == "__main__":
    main()
