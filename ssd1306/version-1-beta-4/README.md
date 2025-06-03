# Integrating deep sleep mode to SSD1306 display
After reasearching the `ssd1306` crate, i found that there is a function called:
```rust
set_display_on(&mut self, on: bool)
```
This function enable us to turn the display on or off meanwhile he display can be drawn to and retains all of its memory even while off.

```rust
#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    //...other code blocks...
    let mut display_on = true;
    let mut next_toggle = Instant::now() + Duration::from_secs(10);

    display.flush().await.unwrap();
    loop {
        //...waveform logic...
        
        let now = Instant::now();

        if display_on && now >= next_toggle {
            display.set_display_on(false).await.unwrap();
            display_on = false;
            next_toggle = now + Duration::from_secs(5);
        } else if !display_on && now >= next_toggle {
            display.set_display_on(true).await.unwrap();
            display_on = true;
            next_toggle = now + Duration::from_secs(5);
        }        
    }
}
```

To integrate it to our display, first we have to understand the logic of how it works: 
## 1. **Start**

* The display starts **on**.
* A variable `next_toggle` is set to **5 seconds from now**.
  → That means: “wait 10 seconds before turning off the display.”

## 2. **Each time the code runs:**

You check the **current time** (`now = Instant::now()`), then:

## If the display is ON and it's time to toggle:

```rust
if display_on && now >= next_toggle
```

* Turn the display **off**.
* Set `display_on = false`.
* Set `next_toggle` to **5 seconds from now**.

## If the display is OFF and it's time to toggle:

```rust
else if !display_on && now >= next_toggle
```

* Turn the display **on**.
* Set `display_on = true`.
* Set `next_toggle` to **5 seconds from now**.

## 3. **Repeat this check in a loop or a timer**
The idea is: keep checking `Instant::now()` in a loop or task — and toggle the display when it's time.

**Video test**

<video src="../Demo-video/version-1-beta-4-video/ssd1306-deep-sleep.mp4" width="480" height="320" controls></video>

* Starts ON
* After 5 seconds → OFF
* Then every 5 seconds → toggles ON/OFF

## Core Logic Summary

```text
If (current time >= next scheduled toggle time) {
    Toggle display;
    Schedule next toggle for 5 seconds later;
}
```

