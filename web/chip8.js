import init, * as c8 from "../pkg/chip8_rs.js";

let active = false;

init().then(() => {
    c8.init_debug();
    
    // handle instruction execution and display
    setInterval(() => {
        if (active) 
            c8.execute_instructions(500 / 30);

        let display_value = c8.get_display_as_ints();

        let canvas = document.getElementById("chip8-out");
        let ctx = canvas.getContext("2d");

        let rectangle_width = canvas.width / 64;
        let rectangle_height = canvas.height / 32;

        for (let y = 0; y < 32; y++) {
            for (let x = 0; x < 64; x++) {
            let i = x + y * 64;
            let pixel = display_value[i];

            ctx.fillStyle = pixel ? "white" : "black";
            ctx.fillRect(rectangle_width * x, rectangle_height * y, rectangle_width, rectangle_height);
            }
        }
    }, 1000 / 30);

    // resetting program button
    document.getElementById("restart_program_btn").addEventListener("click", c8.reset_pc);

    // handle timers and sound
    let audio = document.getElementById("beep_audio");

    setInterval(() => {
        let sound_active = c8.timer_tick_and_get_sound();

        if (sound_active) {
            audio.play();
        } else {
            audio.pause();
        }
    }, 1000 / 60);

    // handle key inputs
    const keys = "x123qweasdzc4rfv";

    let keys_pressed = {};

    function create_keys_status() {
        let keys_status = [];

        for (let i = 0; i < keys.length; i++) {
            keys_status.push(keys_pressed[keys[i]] || 0);
        }

        return keys_status;
    }

    document.body.addEventListener("keydown", (e) => {
        keys_pressed[e.key] = 1;

        c8.update_keys_status(create_keys_status());
    });

    document.body.addEventListener("keyup", (e) => {
        keys_pressed[e.key] = 0;

        c8.update_keys_status(create_keys_status());
    });

});

// handle program uploading
let program_file_select = document.getElementById("chip8_file_select");

program_file_select.onchange = () => {
    let reader = new FileReader;

    reader.onload = (e) => {
        active = false;

        let file_contents = new Uint8Array(e.target.result);
        let program_bin = c8.program_8_to_16(file_contents);

        c8.reset_inst();
        c8.load_program(program_bin);
        
        active = true;
    }

    reader.readAsArrayBuffer(program_file_select.files[0]);
}


