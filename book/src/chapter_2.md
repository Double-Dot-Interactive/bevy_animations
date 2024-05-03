# How it works

`bevy_animations` works by storing animations in its own data pool and starting and managing animations is as easy as sending an event over bevy.

A brief description of how the `bevy_animations` API works is, you start out by adding an animation with custom configuration, next you can add the animation to an entity if you didn't initially, finally you can start the animation by sending an event over bevy and if `bevy_animations` deems the animation as worthy of being played (which is defined by your custom configuration) it will start.

There are a lot of custom configuration properties for each animation, these will be talked about in depth in each animation chapter. There are also a few gotchas and semantics for using the API that will also be reviewed in this book.

## Beta

`bevy_animations` backend for managing animations and sharing data has gone through a few overhauls. There have also been some API overhauls as well, some of which were needed due to backend changes.

Knowing that major API changes occur should be taken into consideration when choosing this library to manage you animations in Bevy.

## [Continue To Next Chapter ->](./chapter_3.md)
