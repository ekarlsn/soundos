# Development


### Downaload speech syntesis files

```bash
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx.json
```

### Serving Your App

Run the following command in the root of your project to start developing for linux:

```bash
dx serve --platform linux
```
