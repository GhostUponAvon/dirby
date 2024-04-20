# Dirby
The Dirby command line tool for speedily creating large amounts of directories


## About

Dirby is a Command line tool built with rust for creating large amounts of folders without out the need to do it by hand, create batch files, etc. Dirby does this by parsing a simple text file that specifies what directories to make and the child directories. 

### How to use Dirby

1. Create a new .txt text file
2. Fill with the desired directories. See [Syntax Rules](#syntax-rules) for information on how to do this.
3. To create the directories execute the command `dirby [input file] [output directory]`. Where "[input file]" is the text file from earlier and "[output directory]" is the root folder to spawn all the directories inside (if the output directory doesn't exist it will be created).

### Syntax Rules
This section details what you are aloud to fill the input text file with.<br>

To specify a directory, simple type its name:<br>

my_directories.txt
```
my folder
```
To specify more directories at the same level just keep adding new lines with new names. But to make a sub-directory of another directory, add a slash before its name and position it one line under the parent directories name. <br>
Here is a simple example:

my_directories.txt
```
my folder
/my child folder
```
Produces: `[output_dir]/my folder/my child folder`
Here is a more complex example:

my_directories.txt
```
foo
/bar
//baz
fizz
```
Produces: `[output_dir]/foo/bar/baz` and `[output_dir]/fizz`

**Rules**
1. The `/` at the beginning of a line denotes a subdirectory; increasing directory depth by more than one at a time by adding more than one slash, will cause and error. See [files that will cause an error](#erroneous-files).
2. Everything that comes after a slash is the name of the folder and is subject to the host operating systems illegal file system names and characters.

## Compatibility
Dirby has been built with Linux, Windows, and MacOS in mind and should support them. Dirby has however not been tested on MacOS, so is not guaranteed to work as expected.

## Erroneous Files
The following is some examples of files that cause errors because they do not follow the [Syntax Rules](#syntax-rules).

1. Contains a directory named `LPT9`, this problem is windows specific and is caused because "LPT9" is one of many reserved windows file names.
```
my folder
/my folder 2
LPT9
my folder 3
```
2. Attempts to increase by more than one slash at a time. This is a program error.
```
my folder
/my folder 2
///my folder 3
```
3. Contains an illegal filesystem character. The illegal is `/` and the reason it causes an error is because it is located specifically in the directory name section of the line.
```
my folder
my folder2/my folder 3
```

# Contributing
There will be a more detailed way to contribute in future, but for now please raise an issue in the issue tab if there is a bug or a feature you would like to request.
