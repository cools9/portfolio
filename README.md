# Cools9's Portfolio

My personal portfolio website made using Rust and Dioxus.

I wanted to try making a website without relying on the usual JavaScript frameworks, so I decided to mess around with Dioxus and see how far I could take it. This project is also a way for me to learn more about Rust's frontend capabilities and how Dioxus handles things like components, state, and web interactions.

The website is pretty simple for now, but I'll probably keep adding random stuff to it as I learn more and get more ideas.

# Features
- About me section
- Project cards
- GitHub links
- Interactive accordion sections
- Responsive layout
- Some mildly unnecessary animations/interactions
- A live age counter that updates every second because why not
- Reusable components for things like cards and accordions

# Made with
- 🦀 Rust — Main programming language
- Dioxus — Used to build the UI
- Tailwind CSS — Styling and layout
- Chrono — Used for calculating the age counter
- Dioxus Icons — Icons used around the site
- Gloo Timers — Used to update the age counter every second

# installation instructions
If you want to run the portfolio yourself, you'll need Rust and the Dioxus CLI installed.

- 1. Clone the repository
```
git clone https://github.com/Cools9/portfolio.git
cd portfolio
```

- 2. Run the project

Once you're inside the project directory, run:
```
dx serve
```

Dioxus should then build the project and start the development server.

Project Structure
I used dioxus compoenets for most of the components as it saved me a hella bit of time

# Why Dioxus?
I mainly wanted to see what building a frontend in Rust would actually be like.
Dioxus lets me write the UI using Rust while still getting things like components, reactive state, and event handling. It's definitely different from making a website with something like React, but that's also what makes it interesting to me.
This portfolio is basically me experimenting with Dioxus while also having an actual project to show for it.

# Future plans
- I'll probably add more projects, improve the design, and add more random features whenever I get an idea.
- There isn't really a strict roadmap for this project — I'm mostly just building things and seeing what works.

Made with LOVE.
