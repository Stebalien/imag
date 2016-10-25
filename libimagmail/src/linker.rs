//! The linker takes an iterator of mails and links them together by their message-id, using
//! `libimaglink`.


generate_error_module!(
    generate_error_types!(LinkerError, LinkerErrorKind,
        LinkerConstructionError => "Error while build()ing the Linker object",
        NoMessageIdFoundError   => "No Message-Id for mail found",
        LinkerError             => "Error while linking mails"
    );
);

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error as FmtError, Result as FmtResult};

use libimagerror::into::IntoError;

use mail::Mail;

use self::error::LinkerError;
use self::error::LinkerErrorKind as LEK;
use self::error::MapErrInto;

bitflags! {
    pub flags LinkerOpts: u32 {
        const IGNORE_IMPORT_NOMSGID  = 0b00000001,
        const IGNORE_IMPORT_REPTOERR = 0b00000010,
        const RETURN_SOON            = 0b00000100,
        const PRINT_INFO             = 0b00001000,
    }
}

impl Display for LinkerOpts {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", flags_to_str(self))
    }
}

fn flags_to_str(flgs: &LinkerOpts) -> &'static str {
    match *flgs {
        IGNORE_IMPORT_NOMSGID  => "Ignore if there was an error while fetching the Message-Id field",
        IGNORE_IMPORT_REPTOERR => "Ignore if there was an error while fetching the In-Reply-To header field",
        RETURN_SOON            => "Return as soon as an error occurs",
        PRINT_INFO             => "Print information if linking succeeded",
        LinkerOpts { .. }      => "Unknown Linker option",
    }
}

type MessageId = String;

#[derive(Debug)]
struct MemMail<'a>(Mail<'a>, Option<MessageId>);

pub struct Linker<'a> {
    h: HashMap<MessageId, MemMail<'a>>,
    flags: LinkerOpts,
}

impl<'a> Linker<'a> {

    pub fn build<I>(i: I, flags: LinkerOpts) -> Result<Linker<'a>, LinkerError>
        where I: Iterator<Item = Mail<'a>>
    {
        let mut h : HashMap<MessageId, MemMail> = HashMap::new();

        for mail in i {
            let id = try!(mail.get_message_id().map_err_into(LEK::LinkerConstructionError));
            let inrepto_id =
                match mail.get_in_reply_to().map_err_into(LEK::LinkerConstructionError) {
                    Ok(r) => r,
                    Err(e) => if flags.contains(IGNORE_IMPORT_REPTOERR) {
                        debug!("Could not retrieve In-Reply-To header of: {:?}", id);
                        None
                    } else {
                        return Err(e)
                    },
                };

            match (id, inrepto_id) {
                (Some(id), Some(repto)) => {
                    debug!("Mail: {} in reply to {}", id, repto);
                    h.insert(id, MemMail(mail, Some(repto)));
                },
                (Some(id), None)        => {
                    debug!("Mail that does not reply to any other mail: {}", id);
                    h.insert(id, MemMail(mail, None));
                },
                (None, _)               => {
                    debug!("No message id for mail: {:?}", mail);
                    if flags.contains(IGNORE_IMPORT_NOMSGID) {
                        debug!("Ignoring no message id");
                    } else {
                        let nomsgid = LEK::NoMessageIdFoundError;
                        let lce = LEK::LinkerConstructionError;

                        return Err(nomsgid.into_error()).map_err_into(lce);
                    }
                },
            }
        }

        h.shrink_to_fit(); // as we won't add things anymore now

        Ok(Linker { h: h, flags: flags })
    }

    /// Run the linker
    ///
    /// Use the LinkerOpts `opts` to configure the linker for this run.
    ///
    /// # Return value
    ///
    /// On error, this returns a LinkerError which can then be transformed into a MailError
    ///
    pub fn run(&mut self) -> Result<(), LinkerError> {
        use libimagentrylink::internal::InternalLinker;

        for (id, &mut MemMail(ref mut mail, ref mut o_replyto)) in self.h.iter_mut() {
            if o_replyto.is_none() {
                continue;
            }
            let o_replyto = o_replyto.as_ref().unwrap();

            let mut other_mail = match self.h.get_mut(&o_replyto) {
                None    => continue,
                Some(o) => o,
            };

            match mail.add_internal_link(&mut other_mail.0).map_err_into(LEK::LinkerError) {
                Ok(_) => if self.flags.contains(PRINT_INFO) {
                    info!("{} -> {}", id, o_replyto);
                } else {
                    debug!("Linking succeeded: {id} -> {other}", id = id, other = o_replyto);
                },

                err => if self.flags.contains(RETURN_SOON) {
                    return err;
                },
            }

        }

        Ok(())
    }

}

