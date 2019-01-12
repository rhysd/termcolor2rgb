extern crate termcolor;

use termcolor::Color;

pub trait ColorExt {
    fn to_rgb(&self) -> (u8, u8, u8);
}

impl ColorExt for Color {
    fn to_rgb(&self) -> (u8, u8, u8) {
        use termcolor::Color::*;
        match self {
            Rgb(r, g, b) => (*r, *g, *b),
            Black => (0x00, 0x00, 0x00),
            Red => (0x80, 0x00, 0x00),
            Green => (0x00, 0x80, 0x00),
            Yellow => (0x80, 0x80, 0x00),
            Blue => (0x00, 0x00, 0x80),
            Magenta => (0x80, 0x00, 0x80),
            Cyan => (0x00, 0x80, 0x80),
            White => (0xc0, 0xc0, 0xc0),
            // Primary 3-bit (8 colors). Unique representation!
            Ansi256(0) => (0x00, 0x00, 0x00),
            Ansi256(1) => (0x80, 0x00, 0x00),
            Ansi256(2) => (0x00, 0x80, 0x00),
            Ansi256(3) => (0x80, 0x80, 0x00),
            Ansi256(4) => (0x00, 0x00, 0x80),
            Ansi256(5) => (0x80, 0x00, 0x80),
            Ansi256(6) => (0x00, 0x80, 0x80),
            Ansi256(7) => (0xc0, 0xc0, 0xc0),
            // Equivalent "bright" versions of original 8 colors.
            Ansi256(8) => (0x80, 0x80, 0x80),
            Ansi256(9) => (0xff, 0x00, 0x00),
            Ansi256(10) => (0x00, 0xff, 0x00),
            Ansi256(11) => (0xff, 0xff, 0x00),
            Ansi256(12) => (0x00, 0x00, 0xff),
            Ansi256(13) => (0xff, 0x00, 0xff),
            Ansi256(14) => (0x00, 0xff, 0xff),
            Ansi256(15) => (0xff, 0xff, 0xff),
            // Strictly ascending.
            Ansi256(16) => (0x00, 0x00, 0x00),
            Ansi256(17) => (0x00, 0x00, 0x5f),
            Ansi256(18) => (0x00, 0x00, 0x87),
            Ansi256(19) => (0x00, 0x00, 0xaf),
            Ansi256(20) => (0x00, 0x00, 0xd7),
            Ansi256(21) => (0x00, 0x00, 0xff),
            Ansi256(22) => (0x00, 0x5f, 0x00),
            Ansi256(23) => (0x00, 0x5f, 0x5f),
            Ansi256(24) => (0x00, 0x5f, 0x87),
            Ansi256(25) => (0x00, 0x5f, 0xaf),
            Ansi256(26) => (0x00, 0x5f, 0xd7),
            Ansi256(27) => (0x00, 0x5f, 0xff),
            Ansi256(28) => (0x00, 0x87, 0x00),
            Ansi256(29) => (0x00, 0x87, 0x5f),
            Ansi256(30) => (0x00, 0x87, 0x87),
            Ansi256(31) => (0x00, 0x87, 0xaf),
            Ansi256(32) => (0x00, 0x87, 0xd7),
            Ansi256(33) => (0x00, 0x87, 0xff),
            Ansi256(34) => (0x00, 0xaf, 0x00),
            Ansi256(35) => (0x00, 0xaf, 0x5f),
            Ansi256(36) => (0x00, 0xaf, 0x87),
            Ansi256(37) => (0x00, 0xaf, 0xaf),
            Ansi256(38) => (0x00, 0xaf, 0xd7),
            Ansi256(39) => (0x00, 0xaf, 0xff),
            Ansi256(40) => (0x00, 0xd7, 0x00),
            Ansi256(41) => (0x00, 0xd7, 0x5f),
            Ansi256(42) => (0x00, 0xd7, 0x87),
            Ansi256(43) => (0x00, 0xd7, 0xaf),
            Ansi256(44) => (0x00, 0xd7, 0xd7),
            Ansi256(45) => (0x00, 0xd7, 0xff),
            Ansi256(46) => (0x00, 0xff, 0x00),
            Ansi256(47) => (0x00, 0xff, 0x5f),
            Ansi256(48) => (0x00, 0xff, 0x87),
            Ansi256(49) => (0x00, 0xff, 0xaf),
            Ansi256(50) => (0x00, 0xff, 0xd7),
            Ansi256(51) => (0x00, 0xff, 0xff),
            Ansi256(52) => (0x5f, 0x00, 0x00),
            Ansi256(53) => (0x5f, 0x00, 0x5f),
            Ansi256(54) => (0x5f, 0x00, 0x87),
            Ansi256(55) => (0x5f, 0x00, 0xaf),
            Ansi256(56) => (0x5f, 0x00, 0xd7),
            Ansi256(57) => (0x5f, 0x00, 0xff),
            Ansi256(58) => (0x5f, 0x5f, 0x00),
            Ansi256(59) => (0x5f, 0x5f, 0x5f),
            Ansi256(60) => (0x5f, 0x5f, 0x87),
            Ansi256(61) => (0x5f, 0x5f, 0xaf),
            Ansi256(62) => (0x5f, 0x5f, 0xd7),
            Ansi256(63) => (0x5f, 0x5f, 0xff),
            Ansi256(64) => (0x5f, 0x87, 0x00),
            Ansi256(65) => (0x5f, 0x87, 0x5f),
            Ansi256(66) => (0x5f, 0x87, 0x87),
            Ansi256(67) => (0x5f, 0x87, 0xaf),
            Ansi256(68) => (0x5f, 0x87, 0xd7),
            Ansi256(69) => (0x5f, 0x87, 0xff),
            Ansi256(70) => (0x5f, 0xaf, 0x00),
            Ansi256(71) => (0x5f, 0xaf, 0x5f),
            Ansi256(72) => (0x5f, 0xaf, 0x87),
            Ansi256(73) => (0x5f, 0xaf, 0xaf),
            Ansi256(74) => (0x5f, 0xaf, 0xd7),
            Ansi256(75) => (0x5f, 0xaf, 0xff),
            Ansi256(76) => (0x5f, 0xd7, 0x00),
            Ansi256(77) => (0x5f, 0xd7, 0x5f),
            Ansi256(78) => (0x5f, 0xd7, 0x87),
            Ansi256(79) => (0x5f, 0xd7, 0xaf),
            Ansi256(80) => (0x5f, 0xd7, 0xd7),
            Ansi256(81) => (0x5f, 0xd7, 0xff),
            Ansi256(82) => (0x5f, 0xff, 0x00),
            Ansi256(83) => (0x5f, 0xff, 0x5f),
            Ansi256(84) => (0x5f, 0xff, 0x87),
            Ansi256(85) => (0x5f, 0xff, 0xaf),
            Ansi256(86) => (0x5f, 0xff, 0xd7),
            Ansi256(87) => (0x5f, 0xff, 0xff),
            Ansi256(88) => (0x87, 0x00, 0x00),
            Ansi256(89) => (0x87, 0x00, 0x5f),
            Ansi256(90) => (0x87, 0x00, 0x87),
            Ansi256(91) => (0x87, 0x00, 0xaf),
            Ansi256(92) => (0x87, 0x00, 0xd7),
            Ansi256(93) => (0x87, 0x00, 0xff),
            Ansi256(94) => (0x87, 0x5f, 0x00),
            Ansi256(95) => (0x87, 0x5f, 0x5f),
            Ansi256(96) => (0x87, 0x5f, 0x87),
            Ansi256(97) => (0x87, 0x5f, 0xaf),
            Ansi256(98) => (0x87, 0x5f, 0xd7),
            Ansi256(99) => (0x87, 0x5f, 0xff),
            Ansi256(100) => (0x87, 0x87, 0x00),
            Ansi256(101) => (0x87, 0x87, 0x5f),
            Ansi256(102) => (0x87, 0x87, 0x87),
            Ansi256(103) => (0x87, 0x87, 0xaf),
            Ansi256(104) => (0x87, 0x87, 0xd7),
            Ansi256(105) => (0x87, 0x87, 0xff),
            Ansi256(106) => (0x87, 0xaf, 0x00),
            Ansi256(107) => (0x87, 0xaf, 0x5f),
            Ansi256(108) => (0x87, 0xaf, 0x87),
            Ansi256(109) => (0x87, 0xaf, 0xaf),
            Ansi256(110) => (0x87, 0xaf, 0xd7),
            Ansi256(111) => (0x87, 0xaf, 0xff),
            Ansi256(112) => (0x87, 0xd7, 0x00),
            Ansi256(113) => (0x87, 0xd7, 0x5f),
            Ansi256(114) => (0x87, 0xd7, 0x87),
            Ansi256(115) => (0x87, 0xd7, 0xaf),
            Ansi256(116) => (0x87, 0xd7, 0xd7),
            Ansi256(117) => (0x87, 0xd7, 0xff),
            Ansi256(118) => (0x87, 0xff, 0x00),
            Ansi256(119) => (0x87, 0xff, 0x5f),
            Ansi256(120) => (0x87, 0xff, 0x87),
            Ansi256(121) => (0x87, 0xff, 0xaf),
            Ansi256(122) => (0x87, 0xff, 0xd7),
            Ansi256(123) => (0x87, 0xff, 0xff),
            Ansi256(124) => (0xaf, 0x00, 0x00),
            Ansi256(125) => (0xaf, 0x00, 0x5f),
            Ansi256(126) => (0xaf, 0x00, 0x87),
            Ansi256(127) => (0xaf, 0x00, 0xaf),
            Ansi256(128) => (0xaf, 0x00, 0xd7),
            Ansi256(129) => (0xaf, 0x00, 0xff),
            Ansi256(130) => (0xaf, 0x5f, 0x00),
            Ansi256(131) => (0xaf, 0x5f, 0x5f),
            Ansi256(132) => (0xaf, 0x5f, 0x87),
            Ansi256(133) => (0xaf, 0x5f, 0xaf),
            Ansi256(134) => (0xaf, 0x5f, 0xd7),
            Ansi256(135) => (0xaf, 0x5f, 0xff),
            Ansi256(136) => (0xaf, 0x87, 0x00),
            Ansi256(137) => (0xaf, 0x87, 0x5f),
            Ansi256(138) => (0xaf, 0x87, 0x87),
            Ansi256(139) => (0xaf, 0x87, 0xaf),
            Ansi256(140) => (0xaf, 0x87, 0xd7),
            Ansi256(141) => (0xaf, 0x87, 0xff),
            Ansi256(142) => (0xaf, 0xaf, 0x00),
            Ansi256(143) => (0xaf, 0xaf, 0x5f),
            Ansi256(144) => (0xaf, 0xaf, 0x87),
            Ansi256(145) => (0xaf, 0xaf, 0xaf),
            Ansi256(146) => (0xaf, 0xaf, 0xd7),
            Ansi256(147) => (0xaf, 0xaf, 0xff),
            Ansi256(148) => (0xaf, 0xd7, 0x00),
            Ansi256(149) => (0xaf, 0xd7, 0x5f),
            Ansi256(150) => (0xaf, 0xd7, 0x87),
            Ansi256(151) => (0xaf, 0xd7, 0xaf),
            Ansi256(152) => (0xaf, 0xd7, 0xd7),
            Ansi256(153) => (0xaf, 0xd7, 0xff),
            Ansi256(154) => (0xaf, 0xff, 0x00),
            Ansi256(155) => (0xaf, 0xff, 0x5f),
            Ansi256(156) => (0xaf, 0xff, 0x87),
            Ansi256(157) => (0xaf, 0xff, 0xaf),
            Ansi256(158) => (0xaf, 0xff, 0xd7),
            Ansi256(159) => (0xaf, 0xff, 0xff),
            Ansi256(160) => (0xd7, 0x00, 0x00),
            Ansi256(161) => (0xd7, 0x00, 0x5f),
            Ansi256(162) => (0xd7, 0x00, 0x87),
            Ansi256(163) => (0xd7, 0x00, 0xaf),
            Ansi256(164) => (0xd7, 0x00, 0xd7),
            Ansi256(165) => (0xd7, 0x00, 0xff),
            Ansi256(166) => (0xd7, 0x5f, 0x00),
            Ansi256(167) => (0xd7, 0x5f, 0x5f),
            Ansi256(168) => (0xd7, 0x5f, 0x87),
            Ansi256(169) => (0xd7, 0x5f, 0xaf),
            Ansi256(170) => (0xd7, 0x5f, 0xd7),
            Ansi256(171) => (0xd7, 0x5f, 0xff),
            Ansi256(172) => (0xd7, 0x87, 0x00),
            Ansi256(173) => (0xd7, 0x87, 0x5f),
            Ansi256(174) => (0xd7, 0x87, 0x87),
            Ansi256(175) => (0xd7, 0x87, 0xaf),
            Ansi256(176) => (0xd7, 0x87, 0xd7),
            Ansi256(177) => (0xd7, 0x87, 0xff),
            Ansi256(178) => (0xd7, 0xaf, 0x00),
            Ansi256(179) => (0xd7, 0xaf, 0x5f),
            Ansi256(180) => (0xd7, 0xaf, 0x87),
            Ansi256(181) => (0xd7, 0xaf, 0xaf),
            Ansi256(182) => (0xd7, 0xaf, 0xd7),
            Ansi256(183) => (0xd7, 0xaf, 0xff),
            Ansi256(184) => (0xd7, 0xd7, 0x00),
            Ansi256(185) => (0xd7, 0xd7, 0x5f),
            Ansi256(186) => (0xd7, 0xd7, 0x87),
            Ansi256(187) => (0xd7, 0xd7, 0xaf),
            Ansi256(188) => (0xd7, 0xd7, 0xd7),
            Ansi256(189) => (0xd7, 0xd7, 0xff),
            Ansi256(190) => (0xd7, 0xff, 0x00),
            Ansi256(191) => (0xd7, 0xff, 0x5f),
            Ansi256(192) => (0xd7, 0xff, 0x87),
            Ansi256(193) => (0xd7, 0xff, 0xaf),
            Ansi256(194) => (0xd7, 0xff, 0xd7),
            Ansi256(195) => (0xd7, 0xff, 0xff),
            Ansi256(196) => (0xff, 0x00, 0x00),
            Ansi256(197) => (0xff, 0x00, 0x5f),
            Ansi256(198) => (0xff, 0x00, 0x87),
            Ansi256(199) => (0xff, 0x00, 0xaf),
            Ansi256(200) => (0xff, 0x00, 0xd7),
            Ansi256(201) => (0xff, 0x00, 0xff),
            Ansi256(202) => (0xff, 0x5f, 0x00),
            Ansi256(203) => (0xff, 0x5f, 0x5f),
            Ansi256(204) => (0xff, 0x5f, 0x87),
            Ansi256(205) => (0xff, 0x5f, 0xaf),
            Ansi256(206) => (0xff, 0x5f, 0xd7),
            Ansi256(207) => (0xff, 0x5f, 0xff),
            Ansi256(208) => (0xff, 0x87, 0x00),
            Ansi256(209) => (0xff, 0x87, 0x5f),
            Ansi256(210) => (0xff, 0x87, 0x87),
            Ansi256(211) => (0xff, 0x87, 0xaf),
            Ansi256(212) => (0xff, 0x87, 0xd7),
            Ansi256(213) => (0xff, 0x87, 0xff),
            Ansi256(214) => (0xff, 0xaf, 0x00),
            Ansi256(215) => (0xff, 0xaf, 0x5f),
            Ansi256(216) => (0xff, 0xaf, 0x87),
            Ansi256(217) => (0xff, 0xaf, 0xaf),
            Ansi256(218) => (0xff, 0xaf, 0xd7),
            Ansi256(219) => (0xff, 0xaf, 0xff),
            Ansi256(220) => (0xff, 0xd7, 0x00),
            Ansi256(221) => (0xff, 0xd7, 0x5f),
            Ansi256(222) => (0xff, 0xd7, 0x87),
            Ansi256(223) => (0xff, 0xd7, 0xaf),
            Ansi256(224) => (0xff, 0xd7, 0xd7),
            Ansi256(225) => (0xff, 0xd7, 0xff),
            Ansi256(226) => (0xff, 0xff, 0x00),
            Ansi256(227) => (0xff, 0xff, 0x5f),
            Ansi256(228) => (0xff, 0xff, 0x87),
            Ansi256(229) => (0xff, 0xff, 0xaf),
            Ansi256(230) => (0xff, 0xff, 0xd7),
            Ansi256(231) => (0xff, 0xff, 0xff),
            // Gray-scale range.
            Ansi256(232) => (0x08, 0x08, 0x08),
            Ansi256(233) => (0x12, 0x12, 0x12),
            Ansi256(234) => (0x1c, 0x1c, 0x1c),
            Ansi256(235) => (0x26, 0x26, 0x26),
            Ansi256(236) => (0x30, 0x30, 0x30),
            Ansi256(237) => (0x3a, 0x3a, 0x3a),
            Ansi256(238) => (0x44, 0x44, 0x44),
            Ansi256(239) => (0x4e, 0x4e, 0x4e),
            Ansi256(240) => (0x58, 0x58, 0x58),
            Ansi256(241) => (0x62, 0x62, 0x62),
            Ansi256(242) => (0x6c, 0x6c, 0x6c),
            Ansi256(243) => (0x76, 0x76, 0x76),
            Ansi256(244) => (0x80, 0x80, 0x80),
            Ansi256(245) => (0x8a, 0x8a, 0x8a),
            Ansi256(246) => (0x94, 0x94, 0x94),
            Ansi256(247) => (0x9e, 0x9e, 0x9e),
            Ansi256(248) => (0xa8, 0xa8, 0xa8),
            Ansi256(249) => (0xb2, 0xb2, 0xb2),
            Ansi256(250) => (0xbc, 0xbc, 0xbc),
            Ansi256(251) => (0xc6, 0xc6, 0xc6),
            Ansi256(252) => (0xd0, 0xd0, 0xd0),
            Ansi256(253) => (0xda, 0xda, 0xda),
            Ansi256(254) => (0xe4, 0xe4, 0xe4),
            Ansi256(255) => (0xee, 0xee, 0xee),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use termcolor::Color;

    #[test]
    fn ansi8colors() {
        use super::ColorExt;
        assert_eq!(Color::Red.to_rgb(), (0x80, 0, 0));
        assert_eq!(Color::White.to_rgb(), (0xc0, 0xc0, 0xc0));
        assert_eq!(Color::Black.to_rgb(), (0, 0, 0));
    }

    #[test]
    fn ansi_rgb_colors() {
        use super::ColorExt;
        assert_eq!(Color::Rgb(0x12, 0x89, 0xef).to_rgb(), (0x12, 0x89, 0xef));
        assert_eq!(Color::Rgb(0xff, 0xff, 0xff).to_rgb(), (0xff, 0xff, 0xff));
        assert_eq!(Color::Rgb(0, 0, 0).to_rgb(), (0, 0, 0));
    }

    #[test]
    fn ansi256colors() {
        use super::ColorExt;
        assert_eq!(Color::Ansi256(223).to_rgb(), (0xff, 0xd7, 0xaf));
        assert_eq!(Color::Ansi256(0).to_rgb(), (0x0, 0x0, 0x0));
        assert_eq!(Color::Ansi256(231).to_rgb(), (0xff, 0xff, 0xff));
        assert_eq!(Color::Ansi256(255).to_rgb(), (0xee, 0xee, 0xee));
    }
}
