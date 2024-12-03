#![no_std]
#![no_main]

pub mod dayrunner;
pub mod aoc_utils;

extern crate alloc;

use core::usize;

use core::mem::MaybeUninit;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use aoc_utils::{logging, read};
use dayrunner::dayrunner::run_day;
use psp::sys::{self, ClearBuffer, TexturePixelFormat, DisplayPixelFormat,};
use psp::vram_alloc::get_vram_allocator;
use psp::{BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};


psp::module!("sample_module", 1, 1);

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);


static SNOWMAN: &str = "\n\n\n\n\n\
\x20                         .------,\n\
\x20           .\\/.          |______|\n\
\x20         _\\_}{_/_       _|_Ll___|_\n\
\x20          / }{ \\       [__________]          .\\/.\n\
\x20           '/\\'        /          \\        _\\_\\/_/_\n\
\x20                      ()  o  o    ()        / /\\ \\\n\
\x20                       \\ ~~~   .  /          '/\\'\n\
\x20                  _\\/   \\ '...'  /    \\/_\n\
\x20                   \\\\   {`------'}    //\n\
\x20                    \\\\  /`---/',`\\\\  //\n\
\x20                     \\/'  o  | |\\ \\`//\n\
\x20                     /'      | | \\/ /\\\n\
\x20        __,. -- ~~ ~|    o   `\\|      |~ ~~ -- . __\n\
\x20                    |                 |\n\
\x20               jgs  \\    o            /\n\
\x20                     `._           _.'\n\
\x20                        ^~- . -  ~^ ";

// static TREE: &str = "\n\n\n\n\n\n\n\n\n\n\n\n\
// \x20                                        *\n\
// \x20                                       /.\\\n\
// \x20                                      /..'\\\n\
// \x20                                      /'.'\\\n\
// \x20                                     /.''.'\\\n\
// \x20                                     /.'.'.\\\n\
// \x20                              \"'\"\"\"\"/'.''.'.\\\"\"'\"'\"\n\
// \x20                                jgs ^^^[_]^^^";

static TREE: &str = "\n\n\n\n\n\n\n\n\
\x20                                         _\\/_
\x20                                          /\\
\x20                                          /\\
\x20                                         /  \\
\x20                                         /~~\\o
\x20                                        /o   \\
\x20                                       /~~*~~~\\
\x20                                      o/    o \\
\x20                                      /~~~~~~~~\\~'
\x20                                     /__*_______\\
\x20                                          ||
\x20                                        \\====/
\x20                                         \\__/";

static SANTA: &str = "\n\n\n\n\
\x20      _jgs_____________________________________ 
\x20     |    ___ ___ _  _ _____ ___      .-\"\"\",   |
\x20     |   / __/ . \\ \\| |_   _| . \\    /____, \\  |
\x20     |   \\__ \\   |  ` | | | |   |   {_____}`{} |
\x20     |   \\___/_|_|_|\\_| |_| |_|_|  (/ . . \\)   |
\x20     |      ___ _    ___ _ _ ___   {`-=^=-`}   |
\x20     |     /  _/ |  | . \\ | | __/  {   `   }   |
\x20     |    |  (_  |__|   | | |__ \\  {       }   |
\x20     |   _ \\___\\____|_|_|___/___/   {     }    |
\x20     |  (_)_______                   `-,-`     |
\x20     |  |/| NORTH | aka: \"St. Nicholas\"        |
\x20     |  |/| POLE  |      \"Kris Kringle\"        |
\x20     |  |/|\"\"\"\"\"\"\"`      \"Father Christmas\"    |
\x20     |  |/|              \"Pere Noel\"           |
\x20     |_________________________________________|";

static ASCII_ART: [&str; 3] = [SNOWMAN, TREE, SANTA];


static DAYS: [&str; 25] = [
    "1", "2", "3", "4", "5",
    "6", "7", "8", "9", "10",
    "11", "12", "13", "14", "15",
    "16", "17", "18", "19", "20",
    "21", "22", "23", "24", "25"
];

static DAYS_INPUT: [&[u8]; 3] = [
    b"./day1/input.txt\0",
    b"./day2/input.txt\0",
    b"./day3/input.txt\0",
];


fn debug_to_screen(text: &str) {
    unsafe {
        sys::sceGuStart(sys::GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
        sys::sceGuClearColor(0x00000000);
        sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::FAST_CLEAR_BIT);
        sys::sceGuFinish();
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);

        sys::sceGuDebugPrint(0, 0, 0xffffffff,  format!("{}\0", text).as_bytes().as_ptr());
        sys::sceGuDebugFlush();

        sys::sceDisplayWaitVblankStart();
        sys::sceGuSwapBuffers();
    }
}

fn run_menu(art_index: usize) -> i8 {
    let mut selection: i8 = 0;  // NOTE: should be mut in future
    let mut last_input: sys::CtrlButtons = sys::CtrlButtons::default();
    let mut running_menu = false;

    loop {
        let mut pad_data: sys::SceCtrlData = sys::SceCtrlData::default();
        unsafe {
            sys::sceCtrlReadBufferPositive(&mut pad_data, 1);
        }
        if (last_input.bits() == pad_data.buttons.bits()) && running_menu {
            continue;
        }

        let mut debug_out: String = "AoC 2024\n\nSelect a day and press (X) to run:\n\n".to_owned();
        for i in 0..DAYS.len() {
            if i == selection.try_into().unwrap() {
                debug_out.push_str(&format!("[{:0>2}]", DAYS[i]));
            } else {
                debug_out.push_str(&format!(" {:0>2} ", DAYS[i]));
            }

            if (i+1) % 5 == 0 {
                debug_out.push_str("\n");
            }
        }
        debug_out.push_str(ASCII_ART[art_index]);

        debug_to_screen(&debug_out);

        let mut next_selection: i8 = -1;
        if pad_data.buttons.contains(sys::CtrlButtons::LEFT) && !last_input.contains(sys::CtrlButtons::LEFT) {
            next_selection = selection - 1;
        }
        if pad_data.buttons.contains(sys::CtrlButtons::RIGHT) && !last_input.contains(sys::CtrlButtons::RIGHT) {
            next_selection = selection + 1;
        }
        if pad_data.buttons.contains(sys::CtrlButtons::DOWN) && !last_input.contains(sys::CtrlButtons::DOWN) {
            next_selection = selection + 5;
        }
        if pad_data.buttons.contains(sys::CtrlButtons::UP) && !last_input.contains(sys::CtrlButtons::CROSS)  {
            next_selection = selection - 5;
        }
        if pad_data.buttons.contains(sys::CtrlButtons::CROSS) && !last_input.contains(sys::CtrlButtons::CROSS)  {
            return selection;
        }
        last_input = pad_data.buttons;
        if next_selection > -1 && next_selection < 25 {
            selection = next_selection;
        }
        running_menu = true;
    }
}

fn psp_main() {
    psp::enable_home_button();
    let logger = logging::AoCLogger::new(String::from("./main.log"));
    logger.log(&format!("[main] logger initiated"));


    logger.log(&format!("[main] initiating psp output"));
    let allocator = get_vram_allocator().unwrap();
    unsafe {
        let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
        let fbp1 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();

        sys::sceGumLoadIdentity();
        sys::sceGuInit();

        sys::sceGuStart(sys::GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x40000] as *mut _);
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
        sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1 as _, BUF_WIDTH as i32);
        sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
        sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuEnable(sys::GuState::ScissorTest);

        sys::sceGumMatrixMode(sys::MatrixMode::View);
        sys::sceGumLoadIdentity();

        sys::sceGumMatrixMode(sys::MatrixMode::Projection);
        sys::sceGumLoadIdentity();
        sys::sceGumOrtho(0.0,480.0,272.0,0.0,-30.0,30.0);

        sys::sceDisplayWaitVblankStart();
        sys::sceGuFinish();
        sys::sceGuSync(sys::GuSyncMode::Finish, sys::GuSyncBehavior::Wait);
        sys::sceGuDisplay(true);
    }

    let mut menu = true;
    let mut action_complete = false;
    let mut last_input: sys::CtrlButtons = sys::CtrlButtons::default();
    let mut selection = 0;
    let mut art_index: u32;
    let mut tick = 0;
    let mtc = &mut psp::sys::SceKernelUtilsMt19937Context{
        count: 0,
        state: [0; 624]
    };

    unsafe {
        psp::sys::sceRtcGetCurrentTick(&mut tick);
        let mut date = MaybeUninit::uninit();
        psp::sys::sceRtcSetTick(date.as_mut_ptr(), &tick);
        let date = date.assume_init();
        psp::sys::sceKernelUtilsMt19937Init(mtc, date.microseconds);
    }

    logger.log(&format!("[main] running menu"));
    loop {
        if menu {
            unsafe { art_index = psp::sys::sceKernelUtilsMt19937UInt(mtc); }
            selection = run_menu(art_index.rem_euclid(3) as usize);
            menu = false;
            action_complete = false;
        }
        else if (0..25).contains(&selection) {
            if !action_complete {
                let input: String;
                if (selection as usize) < DAYS_INPUT.len() {
                    input = read::into_str(DAYS_INPUT[selection as usize]);
                } else {
                    input = String::new();
                }
                logger.log(&format!("[main] running day {}", selection+1));
                debug_to_screen(&format!(
                    "Running day {}\n{}\nPress (o) to exit back to menu...\n",
                    selection + 1,
                    run_day(selection, input)
                ));
                action_complete = true;
            }

            let mut pad_data: sys::SceCtrlData = sys::SceCtrlData::default();
            unsafe {
                sys::sceCtrlReadBufferPositive(&mut pad_data, 1);
            }

            if last_input.bits() == pad_data.buttons.bits() {
                continue;
            }

            if pad_data.buttons.contains(sys::CtrlButtons::CIRCLE) && !last_input.contains(sys::CtrlButtons::CIRCLE)  {
                menu = true;
            }
            last_input = pad_data.buttons;
        }
    }
}
