# Morst
Morst is a translator, trainer, and game for learning morse code.

This project is to practice and learn Rust. Originally this was going to be built on Amethyst game engine, but I have chosen to make it a website and API instead. I chose to use the Rust web framework Rocket for this. 

![](https://www.rust-lang.org/logos/rust-logo-32x32.png) 
<img src="https://rocket.rs/images/logo-boxed.png" width="32">
<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a7/React-icon.svg/1024px-React-icon.svg.png" width="45">


## To Do List
- [x] Translate single char keyboard input into dotdash
- [x] Multi char translate
- [ ] Create a React front end
- [ ] ~~Setup Amethyst basic interface~~
- [ ] Playback dotdash into audio
- [ ] Paste bulk translate
- [ ] Input stream translate


## Future Features
- Virtual Keyboard 
- File translate (upload .txt into dotdash)
- Morse tree visualization




I used some of the functionality from the crate [Mors](https://docs.rs/crate/mors/0.1.1), but chose to write it out myself rather than just use the crate as a whole. 

*alt names -> Rumorse, Morst, Rorse* 