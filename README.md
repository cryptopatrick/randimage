
# Fingerprint
```randimage``` will generate a __fingerprint__ image file of PNG format.  
Each image has a width of 680 pixels and a height of 20 pixels.  

## Example
This is a generated, 680 pixels wide and 20 pixels heigh, generated fingerprint png:
![](https://github.com/cryptopatrick/randimage/blob/master/assets/example.png?raw=true)
<!-- ![example](assets/example.png)-->


## Permutation Space
The image will contain 68*2 = 136 small squares or colors.
Each square contains one RGB color, randomly selected from a color permutation of 
of 256^3 = 16,777,216. Considering that we have 136 such squares in our
image, the total permuation space is 136 * 16,777,216 = 2,281,701,376, or
around 2 billion unique images.

## Usage
Nothing complicated, simply download the binary, install it and then run:
```shell
randimage <name-of-image>.png
```
In that same directory you should find the image file with the given name.

## Next release will contain...
+ A real CLI interface
+ The possibility to set width and height of the image to be generated.
  Please keep in mind that changing the size of the image will not change
  the permutation space, I will likely keep it the same, somehow.
+ (if I can get it to work) Provide an image as a source of colorspace
  to generate the fingerprint from.

## Please, don't hesitate to make suggestions
Just open a Pull Request and make suggestions for things you'd like to see in randimage.
I'll see if it's something that I can implement. Of course, feel free to make make
the suggested change and an PR.
