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
    * [Contributing to the Rust Ecosystem](#contributing-to-the-rust-ecosystem)
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

### Objective
As mentioned in the problem statement, the primary objective of our project is to design and implement a Rust-based collaborative pixel canvas system. Allowing users to modify pixels on a shared digital grid through a web-interface and see those updates rendered in real-time on a physical ESP32 OLED display. What demonstrates this project’s novelty is the very implementation of an OLED display, most projects similar to this leverage on Rust’s web server capabilities to make collaborative canvases on a digital abstract, but our project bridges Rust’s web server capability with Rust’s low-level programming to hardware by synchronizing an interactive online experience with embedded hardware which would provide an excellent use case for Rust and fill in a gap in the current ecosystem.

### Demonstration Strategy
It is in the plans to have a physical demo on presentation day where our classmates will be able to “draw” on the dot matrix module through our web interface for a period of 5 minutes. This interactive demo aims to demonstrate the robustness of our system as the traffic of our system will be increased severalfold. However, this traffic increase also means that the user to user interaction rate will also increase as there are only a limited number of pixels for a growing number of players. We are hoping that this allows for a lively, high-action demo.

### Key Features
1. **Back-end Pixel Management Service**: This is the Rust's back-end server, which saves the current state of the shared canvas and serves API endpoints responsible for receiving pixel updates and broadcasting them to hardware. Users are calling an endpoint from this service whenever they make an action on the website.
- _State management system_
- _RESTful/WebSocket API endpoints_
- _Concurrency and Data Locking_

2. **Graphical User Interface**: This is the front-end for the simple canvas grid to display in a browser. Users will go through this GUI to make changes to the OLED on the hardware.
- _Interactive pixel grid UI_
- _Colour picker and coordinates display_
- _Real-time rendering_
- _Responsive design for both mobile and PC users_

3. **Embedded Hardware Display**: This is the embedded system portion where the digital canvas is represented on the OLED screen of an ESP32. This is the behind-the-scenes code which powers the API to have control to the board. Users can see this code in action when they make a change to a pixel on the canvas online and view it's change reflected on the board.
- _Display driver module (firmware to communicate between ESP32 and OLED)_
- _Pixel rendering engine_
- _Networking connectivity_


### Additional Features
Aside from the key features, we've also added in a couple of additional features that would be nice to have in the project which we might implement later.

4. **Canvas Admin Reset**: Allow the admin to reset the canvas.
5. **Data Persistence**: Storing the canvas state to disk so the canvas can be recovered in case of server restarts.
6. **Session Throttling**: Limiting the update frequency of which users can make changes to the canvas.
7. **Diagnostic Mode**: Iteratively checks all the pixels of the board ensuring the LED functionality. Useful for verifying if the code and hardware is communicating properly.

For our implementation, we are using a 32x32 dot-matrix OLED module as the canvas, and ESP32 development board to control the OLED module as well as communicate with our back-end webserver. There would also be a front-end interface which users can access through their computers which would then relay information to the back-end server where it gets processed and ultimately reflects changes in the OLED screen on the hardware.

### Evaluation Criteria
In order to evaluate the functionality of our project, we've implemented some criteria. These criteria make sure that the project meets its main objective as well as prove that we found a robust, reliable solution to our challenge.

1. **Secure Endpoints**: Whether we use RESTful or WebSocket endpoints, they must prevent malicious pixel updates which could lead to injection attacks.

2. **Proper OLED Screen Functionality**: The ESP32 OLED module must render the pixel data without synchronization errors.

3. **Network Accessibility**: The system must be accessible via a public URL or local network URL, allowing users to connect from their personal devices

4. **Concurrency**: Back-end server must be able to reliably handle at least five users interacting with the canvas at the same time.

### Contributing to the Rust Ecosystem
These features fill a gap in the Rust ecosystem by demonstrating the idea of integrating a full-stack development into one project using one language. From back-end web services to user interfaces and embedded hardware control, we are creating a unified type-safe and memory-sophisticated framework. By doing this project, we are proving rust can be a solution for an end-to-end system meant for performance-critical, concurrent, and networked applications while also introducing a new use case for rust projects that can bring innovation in Rust-based embedded and web integration.

## Tentative Plan
Briefly and concisely, describe how your team plans to achieve the project objective in a matter of weeks, with clear descriptions of responsibilities for each team member in the team. As the duration of the project is quite short, there is no need to include milestones and tentative dates.