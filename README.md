# url_shortener
<h2>Setup</h2>

To use this the kv namespace binding in needs to be changed in wrangler.toml

Generate a new one with
```cmd
wrangler kv:namespace create "db" --preview
```
Then change the id for preview_id in wrangler.toml to your own

Builds use `worker-build --release` with `panic = "abort"` for WebAssembly compatibility.

<h2>Usage</h2>
<h4>ShareX Config</h4>
<img src="https://github.com/X-rays5/url_shortener/raw/master/readme_assets/sharex_config.png">
