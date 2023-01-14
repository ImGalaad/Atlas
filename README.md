# Atlas - Python World Maker 🧭

<hr>

![Software Screenshot](https://raw.githubusercontent.com/violettttte/map-generator-python/main/illustration.png)


<b>Atlas</b> is a very simple program that procedurally generates a map with a random seed.

We use the Simplex Noise to make this natural overview.

<hr>

## What is Perlin/Simplex Noise ? 🌫

[Perlin Noise](https://en.wikipedia.org/wiki/Perlin_noise) is a procedural texture calculator developed by [Ken Perlin](https://en.wikipedia.org/wiki/Ken_Perlin) in 1985. He studied at Harvard University, New York University and now he is a computer scientist, engineer and university professor.

The function has a pseudo-random appearance, and yet its generation is not really random, it is only the seed that is. 

This property allows this texture to be easily controllable. Multiple zoomed-in copies of Perlin Noise can be inserted into mathematical expressions to create a wide variety of procedural textures.

Perlin Noise is well known for its use in the generation of Minecraft worlds as well as in many other video games. So it's its ease to do many things that attracted us.

#### Why Simplex Noise and not Perlin Noise ?

Complexity :
- Perlin_noise : $O(n^2)$
- Simplex_noise : $O(n)$

Lower computational complexity, cost and fewer multiplications makes the a very fast computing speed

#### Wikipedia Links

> https://en.wikipedia.org/wiki/Simplex_noise
> https://en.wikipedia.org/wiki/Perlin_noise
> https://en.wikipedia.org/wiki/Ken_Perlin

<u>We recommend this video which helped us a lot to understand how to generate a terrain :</u>

> https://youtu.be/CSa5O6knuwI

<hr>

## To Do 📋

- 🐛 Remove bugs/errors :
■■■■■□□□□□ 50%
- ⚡️ Optimization :
■■■■■■■■■■□ 95%
<i>For the moment there are no bugs discovered but we leave it for the next versions</i>
- 📐 Support for the size of the open file
- 🎨 Enable/Disable biome overlay
- 📏 Choice the map size at the creation

<hr>

## Installation

#### Update pip
<i>Don't forget to use the latest version of pip, it's not essential for this code but it's always good to update</i>
```
pip install --upgrade pip
```

#### Tkinter
<i>This library is extremely important, all the display is based on</i>
```
pip install tk
```

#### Yml
<i>This library is important because it is used to load the settings of _config.yml</i>
```
pip install pyyaml
```

<hr>

## License ©

#### [MIT License](https://choosealicense.com/licenses/mit/)

Copyright © 2023 [@Zecyl](https://www.github.com/Zecyl) and [@violettttte](https://github.com/violettttte)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
The Software is provided “as is”, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the Software.
</h5>

<hr>

## Authors

- [@violettttte](https://github.com/violettttte) Project creator, User-Interface management (Tkinter), running optimization and file organization
- [@Zecyl](https://www.github.com/Zecyl) Ideas and project development, Perlin Noise handling, and early versions of save/load processing
