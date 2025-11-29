use std::ffi::CString;

#[allow(non_camel_case_types)]
pub enum MessageType {
    P_CONTROL_HARD_RESET_CLIENT_V1 = 1,
    P_CONTROL_HARD_RESET_SERVER_V1,
    P_CONTROL_SOFT_RESET_V1,
    P_CONTROL_V1,
    P_ACK_V1,
    P_DATA_V1,
    P_CONTROL_HARD_RESET_CLIENT_V2,
    P_CONTROL_HARD_RESET_SERVER_V2,
    P_DATA_V2,
    P_CONTROL_HARD_RESET_CLIENT_V3,
}

struct ControlChannelPacket {
    key_method: u8,
    key_source: u8,

    options: Option<CString>,
    username: Option<CString>,
    password: Option<CString>,
}

impl ControlChannelPacket {
    fn new(method: u8) -> ControlChannelPacket {
        ControlChannelPacket { key_method: 0, key_source: 0, options: None, username: None, password: None }
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        let mut prefix = vec![0u8; 4];
        out.append(&mut prefix); // Literal 0 as first 4 bytes

        out.push(self.key_method);
        out.push(self.key_source);

        if self.options.is_some() {
            out.append(self.options.unwrap().into_bytes().as_mut());
        }

        if self.username.is_some() {
            out.append(self.username.unwrap().into_bytes().as_mut());

            if self.password.is_some() {
                out.append(self.password.unwrap().into_bytes().as_mut()); // Might need null terminator?
            }
        }

        return out;
    }

}
