// Copyright (c) 2022, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use windows_sys::core::PCWSTR;
use windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows_sys::Win32::UI::WindowsAndMessaging::MB_APPLMODAL;
use windows_sys::Win32::UI::WindowsAndMessaging::MB_ICONERROR;
use windows_sys::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION;
use windows_sys::Win32::UI::WindowsAndMessaging::MB_ICONWARNING;
use windows_sys::Win32::UI::WindowsAndMessaging::MB_OK;
use windows_sys::Win32::UI::WindowsAndMessaging::MB_SYSTEMMODAL;

use crate::Style;

pub fn show_message_box(title: &str, text: &str, style: Style) -> bool {
    let flags = match style {
        Style::Critical => MB_OK | MB_ICONERROR | MB_SYSTEMMODAL,
        Style::Warning => MB_OK | MB_ICONWARNING | MB_APPLMODAL,
        Style::Info => MB_OK | MB_ICONINFORMATION | MB_APPLMODAL,
    };
    let mut title: Vec<u16> = title.encode_utf16().collect();
    let mut text: Vec<u16> = text.encode_utf16().collect();
    title.push(0x0); //Push the missing null byte!
    text.push(0x0); //Push the missing null byte!
    let title = title.as_ptr() as PCWSTR;
    let text = text.as_ptr() as PCWSTR;
    let res = unsafe { MessageBoxW(0, text, title, flags) };
    if res == 0 {
        false
    } else {
        true
    }
}
