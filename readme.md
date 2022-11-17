# ControLite

This project is yet another light-weighted approach to control _gadgets_ via the [Buttplug](https://github.com/buttplugio/buttplug) library.  
It is designed for _gadgets_ users who are bored by the dull modes provided by the vendors.  

## Features

- Light-weighted
  - Yeah it is just a single file with ~200 LOC.
  - You can read and modify it even if your major is not computer science.
- Random mode:
  - Most vendors do not support random mode in their apps, so you have to switch the patterns manually and get distracted.
  - We pick a random pattern randomly after a random time gap, bringing more excitement while letting your hands free.
  - Highly recommend!
- Customized patterns:
  - You can design your own pattern by writing a tiny function f: time -> \[speed0, speed1\].
  - We recommend using code instead of drawing lines to make your pattern more coherent and accurate.
- Alarm clock (auto on):
  - The idea comes from Magic Motion app, which can set a time to start your _gadgets_. However, on the original app, you cannot switch to other modes or you have to reset your alarm afterwards.
  - On Controlite, you can continue playing.
- Lock (developing):
  - You cannot stop your _gadgets_ before the time is up, enjoy!

... and more on the way.  

Supportted _gadgets_ are listed at https://iostindex.com/?filter0ButtplugSupport=4.

## Usage

### Build

Just run
```
cargo build
```

### Run

Just run
```
cargo run
```

Then a console is displayed. ControLite will automatically detect nearby _gadgets_. If a connection is established, it will vibrate for 1 seconds, then added to the device set. It can support multiple connections at the same time.
You may type commands to select patterns or enter random mode.

```
pattern <p>         // select pattern p (now we have 8 of them, numbered 0 .. 7)
pause               // short for pattern 0, which all vibrators are stopped
high                // short for pattern 1, which all vibrators are at maximum speed
random              // all gadgets go randomly. note: their patterns can be different
alarm <hh:mm:ss>    // set an alarm clock
```

## Contribution

You are very welcomed to submit your ideas to Issues, or your implementations of those ideas as Pull Requests. However, please keep it simple, we aim just to build a fun controller for end-users (such as your girlfriend) instead of an IDE.

You may also wish to join chatgroups.

- Telegram: https://t.me/+-6VSEbVkCP42MDE1
- QQ Group No.: 589966195
