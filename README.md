# maime

Another rethinked Mai2

At a glance, it just code taken from `kmai`. How is this differ from the original game?

## Some brief outlines

### Requirements:
- Clearly have beat
- Judgement point must
	+ Eliminate analog control (this have really ugly and anoying noise to deal with, also not work well with WIP engines like Bevy)
	+ Touch control is a must
	+ Touch control must also have a reasonable boundary (sensors)
~> For inst, maimai
- Judgement point must have a coordination-wise end

### Optional
- Visual fill in a 1:1 square
- Visual fill against a boundary circle (all control)
~> mai

### Style:
- Represent chaotic sequences, controlwise ~> ADOFAI
- Hold notes really have a sense of direction, like playing violins

### Ideas:
- Eye: 
	+ very symmetric, could be use for visual mutation like spining, tilting, etc, shaking-vertically judgement line
	+ also increase visual coordination for keyboards
	+ have a reasonable challenging state: closed eye (maybe half of control will be judged, so we have double button for each "nodes" for this state)
	+ pv display gotta be unique (really atm no idea how to bring lots of decoded buffer among the wild without returning home)
- Memorization (*maybe* represent as the inner-eye, or iris turn full red, or anything that cull the notes away):
	+ (-) can only be used in various track that have a repeated partten, place right each other
- Shuttle, fully closed-off (idea eye.3):
    + 2x control choices
	+ bring in the idea of judgement changing, and mixing between rhythm games???
	+ (-) sdez controls gone wrong

### Controls
    - EYE-tionary: the whole control is dedicated to 
    	+ kbd, ofc the whole eye idea is to bring maimai into keyboard, because sdez itself suck at playing with keyboards
		~ / e - r ^ u - i \ | upper eye
		~ \ d - f v j - k / | lower eye
    - Screen-tionary
        + sdez
    	+ asstrodx

## Whats cool
    - eye is 1-pass, 1-quad and high-quality asf
    - nothing else, comming soon

## TODO

- [ ] add console, to dispatch test animation event that eventually drive the game
  + can really borrow the vim-like console from the guy who show in bevy showcases, i love it 
- [ ] actually make animation mix working (but with shader, never touch world transform, this is 2d!)

## RTFM

- to start, `cargo run --profile dev`
- to actually ship, `cargo run --profile dist` and make sure `assets` is with the binaries