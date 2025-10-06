# Project Proposal

**Project Name:** RustyCanvas, an embedded canvas with web server

**Collaborators:** Rohan Datta, Eric Chiang, Alexander Short

## Table of Contents

* [Motivation](#motivation)
    * [Background](#background)
    * [Challenge](#challenge)
    * [Problem Statement](#problem-statement)
* [Objective and Key Features](#objective-and-key-features)
    * [Objective](#objective)
    * [Demonstration Strategy](#demonstration)
    * [Key Features](#key-features)
    * [Additional Features](#additional-features)
    * [Evaluation Criteria](#evaluation-criteria)
* [Tentative Plan](#tentative-plan)

## Motivation

### Background
One interesting aspect of our team is that two out of three team members are not from a software background, instead, Alex and Eric’s undergraduate background are of electrical engineering, focusing more on circuit design and microcontroller firmware programming. Rohan, our third team member, completed his undergraduate studies in software engineering and specialized in building and testing various web applications with APIs, back-end servers, and front-end interfaces. This balance between hardware-oriented and software-oriented skills motivated us to look for a project that would combine our strengths in a meaningful and impactful manner.

In the last ten years, the cooperation between embedded hardware and web-based software has become especially important, especially for Internet of Things (IoT) and interactive devices. But developing such systems is often a distributed toolchain—JavaScript or Python is typically used for high-level web applications, whereas embedded firmware relies on C or C++. This separation introduces complexity, performance inefficiencies, as well as challenges for cross-platform consistency. Rust, being a new systems programming language, has the sole ability of bridging this gap with safe, concurrent, and efficient development in both worlds. There are very few current real-world examples that demonstrate Rust's full-stack potential—from a browser GUI to an embedded microcontroller. Our project bridges that gap by presenting an interactive cooperative art environment that combines web users with a physical OLED display in an easy to use way through a Rust-based framework, showing how Rust can accommodate both hardware and software development in a single unified framework.

### Challenge
Although Rust has been increasingly popular as a systems programming language, integrating it to work smoothly in both embedded hardware and full-stack web development is still a challenge that remains at large. Projects done so far are centered around either low-level firmware for microcontrollers or high-performance backend systems. Our challenge, therefore, is to demonstrate how Rust can unite these two domains in a way that reduces language fragmentation while offering a clean, stable development experience across the hardware up to the cloud.

Our interest in Rust stems from its emerging popularity in embedded software development, more specifically, using Rust as a systems programming language. We wanted to leverage our electrical/software engineering backgrounds to make a unique project that blended low-level hardware with high level backend server software to fit our team's skillset. As Rust has rapidly grown in both embedded systems and web development, the number of times you will see a project that unifies these two domains is rare. We saw this gap as an opportunity to demonstrate Rust’s versatility across the entire stack (server backend to hardware) but in doing so, if we also write a new pathway for embedded systems engineers to use Rust in their projects requiring IoT (internet of things) hardware, that will be great as well.


### Problem Statement
Additionally, we are all avid internet surfers, so we wanted to replicate a fun internet trend once made famous through the Reddit r/place experiment, where thousands of people collaborated on a shared digital canvas by individually placing pixels. We believe that on a small scale this can also be achievable for a project of this caliper along with a unique twist of operating a physical ESP32 OLED display which will make the project both entertaining and technically challenging.

Our project proposes to replicate the online phenomenon that is Reddit’s r/place. On April 1st 2023, Reddit created the r/place subreddit, where users could place a single pixel on a large collaborative canvas, every 5~20 minutes. Users from different subreddits and online fandoms quickly were able to collaborate together to create large pictures and murals from with thousands of pixels. The final collage from r/place can be seen below. 

![r/place](images/rplace.png)

We are excited about this project because it puts a good use case for using Rust in a coherent low-level hardware, real-time networking, and user interface web design project. This project fits our group because each of us can contribute our unique expertise while also learning new Rust skills. In essence, we are making a collaborative digital art experiment that others can interact with. Most importantly, this project fills a gap in the Rust ecosystem by showing how Rust can act as a middle-ware language for hardware and software collaborative projects.

## Objective and Key Features


## Tentative Plan
Briefly and concisely, describe how your team plans to achieve the project objective in a matter of weeks, with clear descriptions of responsibilities for each team member in the team. As the duration of the project is quite short, there is no need to include milestones and tentative dates.