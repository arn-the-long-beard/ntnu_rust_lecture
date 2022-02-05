# Rust Lecture

### Summary ##

#### 1 - History & Memory Management

#### 2 - Rust Intro

#### 3 - Quirks and features

#### 4 - Multithreading

#### 5 - Popularity and future

#### 6 - Play

#### 7 - Conclusion

### 1 - History & Memory Management 

C is the base

https://www.youtube.com/embed/Fm5Ust7vEhk


```C
#include <stdio.h>
int main() {
printf("Hello, World!");
return 0;
}
```

- What is memory management

C & C++ use manual memory management

```c++

// C++ Program to store GPA of n number of students and display it
// where n is the number of students entered by the user

#include <iostream>
using namespace std;

int main() {
    int num;
    cout << "Enter total number of students: ";
    cin >> num;
    float* ptr;

    // memory allocation of num number of floats
    ptr = new float[num];

    cout << "Enter grades of students." << endl;
    for (int i = 0; i < num; ++i) {
        cout << "Student" << i + 1 << ": ";
        cin >> *(ptr + i);
    }

    cout << "\nDisplaying GPA of students." << endl;
    for (int i = 0; i < num; ++i) {
        cout << "Student" << i + 1 << " :" << *(ptr + i) << endl;
    }

    // ptr memory is released
    delete[] ptr;

    return 0;
}


```
- What will happens if you forget to delete.

https://stackoverflow.com/questions/14987318/what-happens-if-i-dont-delete
  
- [Heap and Stack](https://stackoverflow.com/questions/24891/c-memory-management/24922)

- C++ came in the game : https://www.youtube.com/watch?v=JBjjnqG0BP8

C++ and [its challenges](https://www.toptal.com/c-plus-plus/top-10-common-c-plus-plus-developer-mistakes) 

- Then garbage collector language like Java, C#, python to name few of them

![What is garbage collector](assets/garbage-collector.jpg)

- What's a garbage collector?

- Garbage collector vs Manual allocation
  
- Found this by mistake https://github.com/plasma-umass/memory-landscape
  
![duality](assets/with-great-performance.jpg)

- Golang came in the game 

Is the memory management challenge solved ? 

[Not really](https://www.abetterinternet.org/docs/memory-safety/)

[We can ask Microsoft](https://github.com/microsoft/MSRC-Security-Research/blob/master/presentations/2019_02_BlueHatIL/2019_01%20-%20BlueHatIL%20-%20Trends%2C%20challenge%2C%20and%20shifts%20in%20software%20vulnerability%20mitigation.pdf)

The garbage collector issues with a nice example : [discord](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)

- Then a new challenger arrived, with releases in 2015, 2018 and 2021.

![Languages in meme](assets/meme_languages.png)



### 2 - Rust Intro

- 2006 personal project at Mozilla
  
- From OCaml to [LLVM](https://llvm.org/) based compiler written in Rust

- Very first release in 2015  

- Used in the firefox web browser
  
- Its own [foundation](https://foundation.rust-lang.org/) in 2021 
  
- Most loved languages from 2016 from [stackoverflow survey](https://insights.stackoverflow.com/survey/2020#technology-most-loved-dreaded-and-wanted-languages-loved)

- Companies / products using [Rust in production](https://www.rust-lang.org/production/users)

Discord, Npm, Delivroo, 



### 3 - Quirks and features

#### - Memory safe with :

-> [Ownership](https://doc.rust-lang.org/nomicon/ownership.html) that allows you to: 

**a.** `Mutate` on object/variable so you can `write` with `mut` 



**b.** `Borrow` an object so you can `Read` some of its value to use later with`&`



**c.** `Own` an object in an scope that manages its memory automatically , just pass it as assignment et or function


**d.** Makes the compiler `checks` everything for you, it will be your best enemy first and then your best friend later :D




###### Examples:


**a.** Mutability & Immutability

```rust

struct Company {
  pub name: String,
  /// In Billion
  pub value: u32,
}

fn main() {
  let mut facebook = Company {
    name: "FaceMash".to_string(),
    value: 0,
  };
  
  // <- if we do not use mut here, compiler will say NO
  facebook.name = "Facebook".to_string();
  facebook.value = 900;

  println!("{} new name is cool ", facebook.name);
}
```

**b.** Borrowing data to read it.

```rust


fn display_data(company: &Company) {
  println!("Name : {} ", company.name);
  println!("Market Cap : {} ", company.name);
  println!("Rating {}", rate_business(&company.value))
}

fn main() {
    // ---- Rest of the code
  // 2 - Reference with `&` to read data
  // <- if we do not use & here, compiler will say NO because function asks for it.
  display_data(&facebook);
}

```
**c.** Owning an object to move it ( to consume it ) to do stuff with it. 

```rust


fn rebuild_business(_: Company) -> Company {
  Company {
    name: "Facebook 2.0".to_string(),
    value: 850,
  }
}


fn main() {
  
  // ---- Rest of the code
  println!("{} is super old, we need rebranding ", facebook.name);


  facebook.name = "Meta".to_string();
  println!("{} is an awesome name", facebook.name);

  // <-- we move `facebook` inside the scope of the `rebuild_business` function so we cannot access it anymore .
  let mut new_facebook = rebuild_business(facebook);

  // println!("{} is still alive ?", facebook.name); <-- get moved error value
  display_data(&new_facebook);
  
}


```


**d.** Compiler check for everything for you

Little surprise with mutable references :D

```rust

fn update_name(company: &mut Company, new_name: &str) {
  company.name = new_name.to_string();
}


fn main (){

  // --- Rest of the code ---
  let update = &mut new_facebook; // Can only make a single mutable reference.

  // display_data(&new_facebook); Cannot read while writing :D
  // let update2 = &mut new_facebook;  throw error here because we can only have mutable reference at the same time.
  update.name = "Facebook 3.0".to_string();
  println!("{} is an awesome name.", new_facebook.name);

  let update2 = &mut new_facebook;
  update2.name = "Facebook 4.0".to_string();

  // <- Update name without taking ownership with function
  update_name(&mut new_facebook, "Facebook 3000");
  println!("{} is an awesome name.", new_facebook.name);
}

```

###### Conclusion

Pros :
- Rules to write and read are easy 
- No extra syntax for these rules ( like Malloc or delete)
- Everything is actually automated ( not need to delete object or do memory stuff on basic levels)
- The Compiler checks everything for you
- No need to think about technical detail so you can focus on business logic.

Cons : 
- Redefine the way you write code because you need to turn upside down your brain
- You will hate the compiler
- Get addicted to the safety


Is memory safety solved?

You can do manual [memory management](https://stackoverflow.com/questions/48485454/rust-manual-memory-management) if you want in Unsafe mode




- Traits
  
a. Nice to have for objects that shared common behavior
b. Nice for libraries and use of Generics and have limitations
d. OP for having many types handled together





- Use it everywhere
- Async code
- OMG performances
  https://medium.com/@dexterdarwich/comparison-between-java-go-and-rust-fdb21bd5fb7c

### 4 - Multithreading

https://alexyakunin.medium.com/go-vs-c-part-1-goroutines-vs-async-await-ac909c651c11

Need to add example of Rust

### 5 - Popularity and future

Spotify, Discord, Linux, Aws, Microsoft

How does Rust compete with other languages ( could speak about Go vs Rust for Discord )

https://discord.com/blog/why-discord-is-switching-from-go-to-rust

Why Linux, Android and Windows are switching to Rust now.

https://fossbytes.com/developers-reveal-why-rust-programming-language-is-losing-popularity/
### 6 - Play 

https://github.com/danistefanovic/build-your-own-x

