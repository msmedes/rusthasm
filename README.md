# Assembler for the Hack language

This is an assembler for the `Hack` language created for Nand2Tetris.

I've already written a [python](https://github.com/msmedes/pyhasm) version that I ported to [go](https://github.com/msmedes/gohasm) and now I'm trying to learn `rust` and woo boy.

Some notes: 
- Even though I was just porting the `go`/`python` versions this was still a challenge and I am unsatisfied with the amount of `<some_string>.clone()`'s there are (only 6 or 7 but still).
- The `go` version of this assembler is 50ms faster on the 25k loc `PongL.asm` which is not at all what I expected.  `Go` averages 20ms and the `rust` `release` build takes about 60-70ms.  Why.
- That being said, I spent a lot more time debugging the `go` version, about 10x.  I spent maybe 20 minutes debugging the `rust` version because the compiler caught a lot of mistakes for me.  That being said on that that being said, it also only took me maybe 6 hours to port to `go` and I've lost track of how many hours I've spent porting to `rust` as this is my first ever project in this language.