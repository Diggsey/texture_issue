# Texture Issue
This is an example of the texture issue the stdweb crate. Calling `context.tex_image2_d(...)` should bind texture data to a texture object, but it causes the program to crash because the last parameter does not implement interface ArrayBufferViewOrNull. Full error message: `Argument 9 of WebGLRenderingContext.texImage2D does not implement interface ArrayBufferViewOrNull.`

## How to build and test
### build
`cargo web build`
### test
`cargo web start`