## Overview
This Rust program computes the order of an element $(a,b) \in \in\mathbb{Z}_n \times \in\mathbb{Z}_m$, where $\in\mathbb{Z}_n$ and $\in\mathbb{Z}_m$ are additive groups. The order of an element $(a,b)$ in this direct product is the least common multiple of the orders of $a\in\mathbb{Z}_n$ and $b\in\mathbb{Z}_m$. This order is the smallest positive integer $k$ such that  $(ka, kb) = (0, 0)$.

 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- You can change the values of $a,b,n$ and $m$ in the main function to work with different ones. 
- The program computes and displays the order of $(a,b)\in\mathbb{Z}_n\times\mathbb{Z}_m$.
```bash
git clone https://github.com/cypriansakwa/Order_of_an_Element_of_a_Direct_Product_of_Additive_Groups.git
cd Order_of_an_Element_of_a_Direct_Product_of_Additive_Groups
