# uploader-rs
File uploader with support for multiple hosts and progress reporting written in Rust. Revival of my old Go version.

## Examples
Upload single file to gofile:   
`uploader_x64.exe --hosts gofile -f file.bin`

Upload two files to gofile and pixeldrain:   
`uploader_x64.exe --hosts gofile pixeldrain -f file.bin file2.bin`

Upload all files in `G:\stuff` to gofile recursively and write output template:   
`uploader_x64.exe --hosts gofile -d G:\stuff -r --out-path links.txt`

## Usage
```
Usage: uploader_x64.exe [OPTIONS] --hosts <HOSTS>...

Options:
  -d, --directories <DIRECTORIES>...  
  -f, --file-paths <FILE_PATHS>...    
      --hosts <HOSTS>...              [possible values: fileio, gofile, pixeldrain]
  -o, --out-path <OUT_PATH>           Output template path.
  -r, --recursive                     Include subdirectories.
  -t, --template <TEMPLATE>           Output template. Vars: filename, file_path, host, newline, url. [default: <url><newline>]
  -w, --wipe                          Wipe output template on startup.
  -h, --help                          Print help
```

## Supported hosts
|Host|Argument|Size limit|Requirements|Folder link support|
| --- | --- | --- | --- | --- |
|[file.io](https://www.file.io/)|fileio|2 GB|-|yes, not implemented|
|[Gofile](https://gofile.io/)|gofile|unlim|-|yes|
|[Pixeldrain](https://pixeldrain.com/)|pixeldrain|20 GB, pro: 100 GB|API key, see config|yes, not implemented|
More hosts will be implemented in time, not all hosts are straightforward unfortunately.
