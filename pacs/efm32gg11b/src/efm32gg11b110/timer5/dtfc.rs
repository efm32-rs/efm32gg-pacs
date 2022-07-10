#[doc = "Register `DTFC` reader"]
pub struct R(crate::R<DTFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTFC` writer"]
pub struct W(crate::W<DTFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DTFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DTI PRS Fault Source 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTPRS0FSEL_A {
    #[doc = "0: PRS Channel 0 selected as fault source 0"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as fault source 1"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as fault source 2"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as fault source 3"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as fault source 4"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as fault source 5"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as fault source 6"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as fault source 7"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as fault source 8"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as fault source 9"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as fault source 10"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as fault source 11"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as fault source 12"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as fault source 13"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as fault source 14"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as fault source 15"]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected as fault source 16"]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected as fault source 17"]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected as fault source 18"]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected as fault source 19"]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected as fault source 20"]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected as fault source 21"]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected as fault source 22"]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected as fault source 23"]
    PRSCH23 = 23,
}
impl From<DTPRS0FSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRS0FSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTPRS0FSEL` reader - DTI PRS Fault Source 0 Select"]
pub type DTPRS0FSEL_R = crate::FieldReader<u8, DTPRS0FSEL_A>;
impl DTPRS0FSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTPRS0FSEL_A> {
        match self.bits {
            0 => Some(DTPRS0FSEL_A::PRSCH0),
            1 => Some(DTPRS0FSEL_A::PRSCH1),
            2 => Some(DTPRS0FSEL_A::PRSCH2),
            3 => Some(DTPRS0FSEL_A::PRSCH3),
            4 => Some(DTPRS0FSEL_A::PRSCH4),
            5 => Some(DTPRS0FSEL_A::PRSCH5),
            6 => Some(DTPRS0FSEL_A::PRSCH6),
            7 => Some(DTPRS0FSEL_A::PRSCH7),
            8 => Some(DTPRS0FSEL_A::PRSCH8),
            9 => Some(DTPRS0FSEL_A::PRSCH9),
            10 => Some(DTPRS0FSEL_A::PRSCH10),
            11 => Some(DTPRS0FSEL_A::PRSCH11),
            12 => Some(DTPRS0FSEL_A::PRSCH12),
            13 => Some(DTPRS0FSEL_A::PRSCH13),
            14 => Some(DTPRS0FSEL_A::PRSCH14),
            15 => Some(DTPRS0FSEL_A::PRSCH15),
            16 => Some(DTPRS0FSEL_A::PRSCH16),
            17 => Some(DTPRS0FSEL_A::PRSCH17),
            18 => Some(DTPRS0FSEL_A::PRSCH18),
            19 => Some(DTPRS0FSEL_A::PRSCH19),
            20 => Some(DTPRS0FSEL_A::PRSCH20),
            21 => Some(DTPRS0FSEL_A::PRSCH21),
            22 => Some(DTPRS0FSEL_A::PRSCH22),
            23 => Some(DTPRS0FSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == DTPRS0FSEL_A::PRSCH23
    }
}
#[doc = "Field `DTPRS0FSEL` writer - DTI PRS Fault Source 0 Select"]
pub type DTPRS0FSEL_W<'a> = crate::FieldWriter<'a, u32, DTFC_SPEC, u8, DTPRS0FSEL_A, 5, 0>;
impl<'a> DTPRS0FSEL_W<'a> {
    #[doc = "PRS Channel 0 selected as fault source 0"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 2"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 3"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 4"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 5"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 6"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 7"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as fault source 8"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as fault source 9"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as fault source 10"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as fault source 11"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as fault source 12"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as fault source 13"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as fault source 14"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as fault source 15"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as fault source 16"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as fault source 17"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as fault source 18"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as fault source 19"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as fault source 20"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as fault source 21"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as fault source 22"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as fault source 23"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(DTPRS0FSEL_A::PRSCH23)
    }
}
#[doc = "DTI PRS Fault Source 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTPRS1FSEL_A {
    #[doc = "0: PRS Channel 0 selected as fault source 1"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as fault source 1"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as fault source 1"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as fault source 1"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as fault source 1"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as fault source 1"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as fault source 1"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as fault source 1"]
    PRSCH7 = 7,
    #[doc = "8: PRS Channel 8 selected as fault source 1"]
    PRSCH8 = 8,
    #[doc = "9: PRS Channel 9 selected as fault source 1"]
    PRSCH9 = 9,
    #[doc = "10: PRS Channel 10 selected as fault source 1"]
    PRSCH10 = 10,
    #[doc = "11: PRS Channel 11 selected as fault source 1"]
    PRSCH11 = 11,
    #[doc = "12: PRS Channel 12 selected as fault source 1"]
    PRSCH12 = 12,
    #[doc = "13: PRS Channel 13 selected as fault source 1"]
    PRSCH13 = 13,
    #[doc = "14: PRS Channel 14 selected as fault source 1"]
    PRSCH14 = 14,
    #[doc = "15: PRS Channel 15 selected as fault source 1"]
    PRSCH15 = 15,
    #[doc = "16: PRS Channel 16 selected as fault source 1"]
    PRSCH16 = 16,
    #[doc = "17: PRS Channel 17 selected as fault source 1"]
    PRSCH17 = 17,
    #[doc = "18: PRS Channel 18 selected as fault source 1"]
    PRSCH18 = 18,
    #[doc = "19: PRS Channel 19 selected as fault source 1"]
    PRSCH19 = 19,
    #[doc = "20: PRS Channel 20 selected as fault source 1"]
    PRSCH20 = 20,
    #[doc = "21: PRS Channel 21 selected as fault source 1"]
    PRSCH21 = 21,
    #[doc = "22: PRS Channel 22 selected as fault source 1"]
    PRSCH22 = 22,
    #[doc = "23: PRS Channel 23 selected as fault source 1"]
    PRSCH23 = 23,
}
impl From<DTPRS1FSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRS1FSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTPRS1FSEL` reader - DTI PRS Fault Source 1 Select"]
pub type DTPRS1FSEL_R = crate::FieldReader<u8, DTPRS1FSEL_A>;
impl DTPRS1FSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTPRS1FSEL_A> {
        match self.bits {
            0 => Some(DTPRS1FSEL_A::PRSCH0),
            1 => Some(DTPRS1FSEL_A::PRSCH1),
            2 => Some(DTPRS1FSEL_A::PRSCH2),
            3 => Some(DTPRS1FSEL_A::PRSCH3),
            4 => Some(DTPRS1FSEL_A::PRSCH4),
            5 => Some(DTPRS1FSEL_A::PRSCH5),
            6 => Some(DTPRS1FSEL_A::PRSCH6),
            7 => Some(DTPRS1FSEL_A::PRSCH7),
            8 => Some(DTPRS1FSEL_A::PRSCH8),
            9 => Some(DTPRS1FSEL_A::PRSCH9),
            10 => Some(DTPRS1FSEL_A::PRSCH10),
            11 => Some(DTPRS1FSEL_A::PRSCH11),
            12 => Some(DTPRS1FSEL_A::PRSCH12),
            13 => Some(DTPRS1FSEL_A::PRSCH13),
            14 => Some(DTPRS1FSEL_A::PRSCH14),
            15 => Some(DTPRS1FSEL_A::PRSCH15),
            16 => Some(DTPRS1FSEL_A::PRSCH16),
            17 => Some(DTPRS1FSEL_A::PRSCH17),
            18 => Some(DTPRS1FSEL_A::PRSCH18),
            19 => Some(DTPRS1FSEL_A::PRSCH19),
            20 => Some(DTPRS1FSEL_A::PRSCH20),
            21 => Some(DTPRS1FSEL_A::PRSCH21),
            22 => Some(DTPRS1FSEL_A::PRSCH22),
            23 => Some(DTPRS1FSEL_A::PRSCH23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH11
    }
    #[doc = "Checks if the value of the field is `PRSCH12`"]
    #[inline(always)]
    pub fn is_prsch12(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH12
    }
    #[doc = "Checks if the value of the field is `PRSCH13`"]
    #[inline(always)]
    pub fn is_prsch13(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH13
    }
    #[doc = "Checks if the value of the field is `PRSCH14`"]
    #[inline(always)]
    pub fn is_prsch14(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH14
    }
    #[doc = "Checks if the value of the field is `PRSCH15`"]
    #[inline(always)]
    pub fn is_prsch15(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH15
    }
    #[doc = "Checks if the value of the field is `PRSCH16`"]
    #[inline(always)]
    pub fn is_prsch16(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH16
    }
    #[doc = "Checks if the value of the field is `PRSCH17`"]
    #[inline(always)]
    pub fn is_prsch17(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH17
    }
    #[doc = "Checks if the value of the field is `PRSCH18`"]
    #[inline(always)]
    pub fn is_prsch18(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH18
    }
    #[doc = "Checks if the value of the field is `PRSCH19`"]
    #[inline(always)]
    pub fn is_prsch19(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH19
    }
    #[doc = "Checks if the value of the field is `PRSCH20`"]
    #[inline(always)]
    pub fn is_prsch20(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH20
    }
    #[doc = "Checks if the value of the field is `PRSCH21`"]
    #[inline(always)]
    pub fn is_prsch21(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH21
    }
    #[doc = "Checks if the value of the field is `PRSCH22`"]
    #[inline(always)]
    pub fn is_prsch22(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH22
    }
    #[doc = "Checks if the value of the field is `PRSCH23`"]
    #[inline(always)]
    pub fn is_prsch23(&self) -> bool {
        *self == DTPRS1FSEL_A::PRSCH23
    }
}
#[doc = "Field `DTPRS1FSEL` writer - DTI PRS Fault Source 1 Select"]
pub type DTPRS1FSEL_W<'a> = crate::FieldWriter<'a, u32, DTFC_SPEC, u8, DTPRS1FSEL_A, 5, 8>;
impl<'a> DTPRS1FSEL_W<'a> {
    #[doc = "PRS Channel 0 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH7)
    }
    #[doc = "PRS Channel 8 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH8)
    }
    #[doc = "PRS Channel 9 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH9)
    }
    #[doc = "PRS Channel 10 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH10)
    }
    #[doc = "PRS Channel 11 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH11)
    }
    #[doc = "PRS Channel 12 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch12(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH12)
    }
    #[doc = "PRS Channel 13 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch13(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH13)
    }
    #[doc = "PRS Channel 14 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch14(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH14)
    }
    #[doc = "PRS Channel 15 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch15(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH15)
    }
    #[doc = "PRS Channel 16 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch16(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH16)
    }
    #[doc = "PRS Channel 17 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch17(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH17)
    }
    #[doc = "PRS Channel 18 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch18(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH18)
    }
    #[doc = "PRS Channel 19 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch19(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH19)
    }
    #[doc = "PRS Channel 20 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch20(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH20)
    }
    #[doc = "PRS Channel 21 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch21(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH21)
    }
    #[doc = "PRS Channel 22 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch22(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH22)
    }
    #[doc = "PRS Channel 23 selected as fault source 1"]
    #[inline(always)]
    pub fn prsch23(self) -> &'a mut W {
        self.variant(DTPRS1FSEL_A::PRSCH23)
    }
}
#[doc = "DTI Fault Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTFA_A {
    #[doc = "0: No action on fault"]
    NONE = 0,
    #[doc = "1: Set outputs inactive"]
    INACTIVE = 1,
    #[doc = "2: Clear outputs"]
    CLEAR = 2,
    #[doc = "3: Tristate outputs"]
    TRISTATE = 3,
}
impl From<DTFA_A> for u8 {
    #[inline(always)]
    fn from(variant: DTFA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTFA` reader - DTI Fault Action"]
pub type DTFA_R = crate::FieldReader<u8, DTFA_A>;
impl DTFA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTFA_A {
        match self.bits {
            0 => DTFA_A::NONE,
            1 => DTFA_A::INACTIVE,
            2 => DTFA_A::CLEAR,
            3 => DTFA_A::TRISTATE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTFA_A::NONE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DTFA_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DTFA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TRISTATE`"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == DTFA_A::TRISTATE
    }
}
#[doc = "Field `DTFA` writer - DTI Fault Action"]
pub type DTFA_W<'a> = crate::FieldWriterSafe<'a, u32, DTFC_SPEC, u8, DTFA_A, 2, 16>;
impl<'a> DTFA_W<'a> {
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTFA_A::NONE)
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DTFA_A::INACTIVE)
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DTFA_A::CLEAR)
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut W {
        self.variant(DTFA_A::TRISTATE)
    }
}
#[doc = "Field `DTPRS0FEN` reader - DTI PRS 0 Fault Enable"]
pub type DTPRS0FEN_R = crate::BitReader<bool>;
#[doc = "Field `DTPRS0FEN` writer - DTI PRS 0 Fault Enable"]
pub type DTPRS0FEN_W<'a> = crate::BitWriter<'a, u32, DTFC_SPEC, bool, 24>;
#[doc = "Field `DTPRS1FEN` reader - DTI PRS 1 Fault Enable"]
pub type DTPRS1FEN_R = crate::BitReader<bool>;
#[doc = "Field `DTPRS1FEN` writer - DTI PRS 1 Fault Enable"]
pub type DTPRS1FEN_W<'a> = crate::BitWriter<'a, u32, DTFC_SPEC, bool, 25>;
#[doc = "Field `DTDBGFEN` reader - DTI Debugger Fault Enable"]
pub type DTDBGFEN_R = crate::BitReader<bool>;
#[doc = "Field `DTDBGFEN` writer - DTI Debugger Fault Enable"]
pub type DTDBGFEN_W<'a> = crate::BitWriter<'a, u32, DTFC_SPEC, bool, 26>;
#[doc = "Field `DTLOCKUPFEN` reader - DTI Lockup Fault Enable"]
pub type DTLOCKUPFEN_R = crate::BitReader<bool>;
#[doc = "Field `DTLOCKUPFEN` writer - DTI Lockup Fault Enable"]
pub type DTLOCKUPFEN_W<'a> = crate::BitWriter<'a, u32, DTFC_SPEC, bool, 27>;
impl R {
    #[doc = "Bits 0:4 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&self) -> DTPRS0FSEL_R {
        DTPRS0FSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&self) -> DTPRS1FSEL_R {
        DTPRS1FSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&self) -> DTFA_R {
        DTFA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&self) -> DTPRS0FEN_R {
        DTPRS0FEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&self) -> DTPRS1FEN_R {
        DTPRS1FEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&self) -> DTDBGFEN_R {
        DTDBGFEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&self) -> DTLOCKUPFEN_R {
        DTLOCKUPFEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - DTI PRS Fault Source 0 Select"]
    #[inline(always)]
    pub fn dtprs0fsel(&mut self) -> DTPRS0FSEL_W {
        DTPRS0FSEL_W::new(self)
    }
    #[doc = "Bits 8:12 - DTI PRS Fault Source 1 Select"]
    #[inline(always)]
    pub fn dtprs1fsel(&mut self) -> DTPRS1FSEL_W {
        DTPRS1FSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&mut self) -> DTFA_W {
        DTFA_W::new(self)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&mut self) -> DTPRS0FEN_W {
        DTPRS0FEN_W::new(self)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&mut self) -> DTPRS1FEN_W {
        DTPRS1FEN_W::new(self)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&mut self) -> DTDBGFEN_W {
        DTDBGFEN_W::new(self)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&mut self) -> DTLOCKUPFEN_W {
        DTLOCKUPFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTI Fault Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtfc](index.html) module"]
pub struct DTFC_SPEC;
impl crate::RegisterSpec for DTFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtfc::R](R) reader structure"]
impl crate::Readable for DTFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtfc::W](W) writer structure"]
impl crate::Writable for DTFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTFC to value 0"]
impl crate::Resettable for DTFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}