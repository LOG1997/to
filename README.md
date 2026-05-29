# to: 你的快捷命令管理器

[使用rust构建] 配置你的快捷命令，在命令行快速的打开网页、应用、文件夹或者文件。

## 命令

### 1. 添加: 

  向配置文件中添加命令

  to add [cmd_name] [cmd_value]

  比如: `to add github https://github.com`、`to add host /etc/hosts`,如果我想快捷打开翻译，以百度地图为例，可以这样配置`to add fy https://fanyi.baidu.com/mtpe-individual/transText\?query\=\{\}`

### 2. 删除:

  从配置文件中删除指定命令

  to del [cmd_name]

  比如: `to del github`

### 3. 列表:

  打印出所有配置好的命令列表

  to ls

### 4. 查找:

  查找命令，会从name和value中查找

  to search [params]

  比如: `to search github`

### 5. 编辑配置文件:

  使用系统默认应用直接打开配置文件，如果想要使用vim打开，请在后面添加`--vim`参数

  to edit

  比如: `to edit`、`to edit --vim`

### 6. 该软件的信息

  to about
