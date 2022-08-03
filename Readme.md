# （WIP）MeS : light markup language for Japanese Scenario.

MeSは日本語シナリオの記述に特化した軽量記述言語です。  
現在、開発途中です。  
MeSの記法等に関する詳細は以下を参照してください。

https://github.com/iranika/MeS/wiki


## Demo

MeSを利用した検証用エディタを下記で公開しています。  

https://mes.iranika.info

## Download

### Install Using Scoop(Windows)

[Scoop](https://scoop.sh/)はWindows向けのパッケージマネージャーです。  
Scoopを使うとMeS CLIのダウンロードとアップデートが簡単にできます。  

scoop本体のインストール(PowerShell)
```ps1
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser # Optional: Needed to run a remote script the first time
irm get.scoop.sh | iex
```

MeSのインストール
```ps1
scoop install https://raw.githubusercontent.com/iranika/MeS/main/scoop/mes.json

```

### Other

実行ファイルとWASMは[Releaseページ](https://github.com/iranika/MeS/releases)に公開されています。
必要に応じて、そこからダウンロードして使ってください。  

## Build

### CLI tool
```
cargo build
```


### wasm-pack

artifactは`./pkg` に出力されます。  
```
wasm-pack --target web
```

## Tests

```
cargo test -- --nocapture
```

## Tips

### PowerShellにおけるOutputEncoding

PowerShellでは標準出力のエンコーディングがShiftJISになっている可能性があります。  
mes.exe単体では文字化けしていないのに、PowerShellの機能(リダイレクトや変数への格納)を経由するタイミングで文字化けするケースがあります。  
これはPowerShellでmesの出力データを扱う際に致命的な問題になります。  
以下で
確認して、必要に応じてUTF-8に設定してください。  

```
## エンコードを確認する
> [console]::OutputEncoding

BodyName          : iso-2022-jp
EncodingName      : 日本語 (シフト JIS)
HeaderName        : iso-2022-jp
WebName           : shift_jis
WindowsCodePage   : 932
IsBrowserDisplay  : True
IsBrowserSave     : True
IsMailNewsDisplay : True
IsMailNewsSave    : True
IsSingleByte      : False
EncoderFallback   : System.Text.InternalEncoderBestFitFallback
DecoderFallback   : System.Text.InternalDecoderBestFitFallback
IsReadOnly        : False
CodePage          : 932


## エンコーディングをUTF8にする
> [console]::OutputEncoding = [Text.Encoding]::UTF8

## エンコーディングをShiftJISに戻す
[console]::OutputEncoding = [Text.Encoding]::GetEncoding('shift_jis')

```