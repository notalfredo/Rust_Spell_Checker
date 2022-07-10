# Rust Spell Checker

Spell checker is a Rust CLI program that will spell check your sentences. 

## Downloading

Use git to clone the repo

```bash
git clone https://github.com/notalfredo/Rust_Spell_Checker.git
```

## How to use

```
cd into directory
```

```
cargo run
```


**should see the following**


```

=====================WELCOME TO MY SPELL CHECK PROGRAM=====================

FORMAT RULES : 
  - Any letter from A-Z is allowed lowercase and uppercase
  - List of symbols to be recognized as separators: space (one white space)
          comma, dot, exclamation mark, question mark (, .!?).
          NO OTHER SEPARATOR CAN BE USED
  - ALL sentances must end with a recognized separator

Enter string to be spell - checked: 
```

**Formating**

As mentioned above the only allowed separators are spaces , commas , dots , question marks , exclamation marks ALL sentences must end with any of these.

**Examples of strings that will work**
```
The separators are    !., ? There is now separator here at the end.
```
```
My favorite annimmals ar cats,dogs,horses.However I like mani other.... HoW about yu???
```
```
cavt. 
```

**Examples of strings that will NOT work**

```
no ? separftfor at the end 
```
```
cavt
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.



## License
[MIT](https://choosealicense.com/licenses/mit/)