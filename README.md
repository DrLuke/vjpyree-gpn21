# vjpyree-gpn21
😎 shady business inc.


## How to run
Download roboto font and add the following files:
```
assets/fonts/Roboto-Black.ttf
assets/fonts/Roboto-Bold.ttf
assets/fonts/Roboto-Regular.ttf
```

and then

```cargo run --release```

## Beat detection

Install [aubio-beat-osc](https://pypi.org/project/aubio-beat-osc/) and run

```
aubio-beat-osc list
aubio-beat-osc beat -c 127.0.0.1 31337 /beat -d YOUR_DEVICE_NUM_HERE -v
```

And then check the `Beat` checkboxes to make it react.
Hold spacebar to disable audio reactivity temporarily.