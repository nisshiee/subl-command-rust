subl-command-rust
====================

A little intelligent `subl` command wrapper.

This wrapper searches the `*.sublime-project` file from current directory and pass it to default command. You have to do only `cd projectdir` and `subl`.


Installation
--------------------

  1.  Install rust programming environment.

      We recommend to use [rustup](https://www.rustup.rs/).

  2.  Clone repository

      ```sh
      git clone https://github.com/nisshiee/subl-command-rust.git
      ```

  3.  Build

      ```sh
      cd subl-command-rust

      vi config/subl_path
      # write your original subl command path
      # see: config/subl_path.sample

      cargo build --release
      ```

  4.  Install

      ```sh
      cp target/release/subl $HOME/bin/
      # or any place in your $PATH
      ```


Usage
--------------------

For example, your project is placed in `~/src/your-awesome-project` and it contains `your-awesome-project.sublime-project` in. You only have to move into this directory and call our wrapper without arguments.

```sh
cd ~/src/your-awesome-project
subl
```

Then Sublime Text creates new window with loading your project settings.


License
--------------------

Available as open source under the terms of the MIT License.

see: [LICENSE](https://github.com/nisshiee/subl-command-rust/blob/master/LICENSE)
