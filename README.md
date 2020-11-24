最初に適当なファイルを作ります（名前や拡張子は自由です）.  
ファイルの1行目に抽出したい部分列を正規表現で表します.  

```
pattern_regex
#SECTION#
hoge_0
hoge_1
hoge_2
...
data_n
#SECTION#
fuga_0
fuga_1
fuga_2
...
fuga_n
```

その後ろに、比較したい2つのデータを入力します.  
また、それらの区切りは `#SECTION#` で表します.  

```
$ cargo run file_path
```

パスを渡して実行すると差分が表示されます.  
