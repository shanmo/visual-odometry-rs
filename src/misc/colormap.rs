// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Colormap helpers for visualizations.

/// Viridis colormap in an RGB u8 vector.
pub fn viridis_u8() -> Vec<(u8, u8, u8)> {
    viridis()
        .into_iter()
        .map(|(r, g, b)| (to_u8(r), to_u8(g), to_u8(b)))
        .collect()
}

fn to_u8(x: f32) -> u8 {
    (255.0 * x).round() as u8
}

/// Viridis colormap in an RGB f32 ([0-1]) vector.
pub fn viridis() -> Vec<(f32, f32, f32)> {
    vec![
        (0.2670040, 0.0048743, 0.3294151),
        (0.2685104, 0.0096048, 0.3354265),
        (0.2699438, 0.0146249, 0.3413789),
        (0.2713048, 0.0199418, 0.3472686),
        (0.2725938, 0.0255630, 0.3530930),
        (0.2738093, 0.0314974, 0.3588525),
        (0.2749524, 0.0377518, 0.3645432),
        (0.2760223, 0.0441672, 0.3701641),
        (0.2770184, 0.0503443, 0.3757145),
        (0.2779414, 0.0563244, 0.3811907),
        (0.2787906, 0.0621453, 0.3865920),
        (0.2795655, 0.0678358, 0.3919172),
        (0.2802665, 0.0734172, 0.3971634),
        (0.2808935, 0.0789070, 0.4023294),
        (0.2814458, 0.0843197, 0.4074140),
        (0.2819235, 0.0896662, 0.4124152),
        (0.2823273, 0.0949554, 0.4173308),
        (0.2826563, 0.1001957, 0.4221603),
        (0.2829104, 0.1053934, 0.4269020),
        (0.2830909, 0.1105530, 0.4315537),
        (0.2831970, 0.1156796, 0.4361148),
        (0.2832288, 0.1207770, 0.4405840),
        (0.2831868, 0.1258479, 0.4449600),
        (0.2830720, 0.1308947, 0.4492412),
        (0.2828838, 0.1359200, 0.4534273),
        (0.2826229, 0.1409255, 0.4575172),
        (0.2822903, 0.1459123, 0.4615099),
        (0.2818867, 0.1508814, 0.4654047),
        (0.2814122, 0.1558342, 0.4692012),
        (0.2808677, 0.1607713, 0.4728990),
        (0.2802546, 0.1656927, 0.4764976),
        (0.2795739, 0.1705988, 0.4799967),
        (0.2788261, 0.1754902, 0.4833965),
        (0.2780123, 0.1803668, 0.4866970),
        (0.2771343, 0.1852283, 0.4898983),
        (0.2761937, 0.1900744, 0.4930007),
        (0.2751911, 0.1949054, 0.4960048),
        (0.2741280, 0.1997208, 0.4989113),
        (0.2730059, 0.2045204, 0.5017207),
        (0.2718281, 0.2093030, 0.5044341),
        (0.2705947, 0.2140689, 0.5070524),
        (0.2693075, 0.2188178, 0.5095767),
        (0.2679684, 0.2235491, 0.5120084),
        (0.2665798, 0.2282621, 0.5143487),
        (0.2651445, 0.2329559, 0.5165993),
        (0.2636632, 0.2376307, 0.5187616),
        (0.2621380, 0.2422861, 0.5208373),
        (0.2605710, 0.2469217, 0.5228282),
        (0.2589645, 0.2515368, 0.5247360),
        (0.2573224, 0.2561304, 0.5265633),
        (0.2556451, 0.2607028, 0.5283115),
        (0.2539349, 0.2652538, 0.5299827),
        (0.2521940, 0.2697830, 0.5315790),
        (0.2504246, 0.2742902, 0.5331026),
        (0.2486289, 0.2787750, 0.5345556),
        (0.2468114, 0.2832366, 0.5359409),
        (0.2449720, 0.2876754, 0.5372601),
        (0.2431132, 0.2920915, 0.5385156),
        (0.2412370, 0.2964847, 0.5397094),
        (0.2393457, 0.3008549, 0.5408439),
        (0.2374413, 0.3052022, 0.5419214),
        (0.2355260, 0.3095265, 0.5429439),
        (0.2336027, 0.3138277, 0.5439142),
        (0.2316735, 0.3181058, 0.5448344),
        (0.2297392, 0.3223612, 0.5457063),
        (0.2278019, 0.3265943, 0.5465320),
        (0.2258633, 0.3308051, 0.5473135),
        (0.2239251, 0.3349940, 0.5480529),
        (0.2219891, 0.3391611, 0.5487521),
        (0.2200569, 0.3433068, 0.5494130),
        (0.2181299, 0.3474315, 0.5500375),
        (0.2162097, 0.3515354, 0.5506274),
        (0.2142975, 0.3556190, 0.5511844),
        (0.2123947, 0.3596827, 0.5517101),
        (0.2105031, 0.3637267, 0.5522064),
        (0.2086234, 0.3677515, 0.5526748),
        (0.2067562, 0.3717577, 0.5531165),
        (0.2049025, 0.3757458, 0.5535328),
        (0.2030630, 0.3797164, 0.5539250),
        (0.2012385, 0.3836698, 0.5542944),
        (0.1994295, 0.3876067, 0.5546420),
        (0.1976365, 0.3915276, 0.5549690),
        (0.1958599, 0.3954329, 0.5552763),
        (0.1941000, 0.3993233, 0.5555649),
        (0.1923571, 0.4031993, 0.5558355),
        (0.1906313, 0.4070614, 0.5560890),
        (0.1889225, 0.4109103, 0.5563260),
        (0.1872308, 0.4147464, 0.5565471),
        (0.1855559, 0.4185704, 0.5567529),
        (0.1838976, 0.4223827, 0.5569437),
        (0.1822556, 0.4261840, 0.5571201),
        (0.1806294, 0.4299748, 0.5572822),
        (0.1790187, 0.4337557, 0.5574303),
        (0.1774229, 0.4375272, 0.5575646),
        (0.1758414, 0.4412898, 0.5576852),
        (0.1742736, 0.4450441, 0.5577921),
        (0.1727187, 0.4487906, 0.5578853),
        (0.1711761, 0.4525298, 0.5579646),
        (0.1696457, 0.4562620, 0.5580303),
        (0.1681264, 0.4599880, 0.5580819),
        (0.1666171, 0.4637081, 0.5581191),
        (0.1651170, 0.4674229, 0.5581414),
        (0.1636254, 0.4711327, 0.5581484),
        (0.1621415, 0.4748382, 0.5581396),
        (0.1606646, 0.4785396, 0.5581146),
        (0.1591941, 0.4822374, 0.5580728),
        (0.1577293, 0.4859319, 0.5580134),
        (0.1562697, 0.4896237, 0.5579360),
        (0.1548148, 0.4933129, 0.5578396),
        (0.1533644, 0.4970000, 0.5577237),
        (0.1519182, 0.5006852, 0.5575873),
        (0.1504760, 0.5043690, 0.5574296),
        (0.1490391, 0.5080513, 0.5572505),
        (0.1476073, 0.5117326, 0.5570486),
        (0.1461802, 0.5154131, 0.5568227),
        (0.1447586, 0.5190931, 0.5565718),
        (0.1433432, 0.5227729, 0.5562949),
        (0.1419352, 0.5264525, 0.5559909),
        (0.1405359, 0.5301321, 0.5556589),
        (0.1391470, 0.5338120, 0.5552977),
        (0.1377704, 0.5374921, 0.5549062),
        (0.1364085, 0.5411726, 0.5544833),
        (0.1350656, 0.5448533, 0.5540290),
        (0.1337429, 0.5485345, 0.5535410),
        (0.1324440, 0.5522163, 0.5530182),
        (0.1311724, 0.5558987, 0.5524594),
        (0.1299327, 0.5595816, 0.5518635),
        (0.1287293, 0.5632650, 0.5512292),
        (0.1275677, 0.5669489, 0.5505555),
        (0.1264533, 0.5706331, 0.5498411),
        (0.1253938, 0.5743175, 0.5490856),
        (0.1243947, 0.5780020, 0.5482874),
        (0.1234628, 0.5816866, 0.5474449),
        (0.1226056, 0.5853710, 0.5465572),
        (0.1218312, 0.5890552, 0.5456229),
        (0.1211480, 0.5927388, 0.5446411),
        (0.1205650, 0.5964218, 0.5436105),
        (0.1200915, 0.6001038, 0.5425304),
        (0.1197375, 0.6037845, 0.5413999),
        (0.1195116, 0.6074638, 0.5402175),
        (0.1194234, 0.6111414, 0.5389819),
        (0.1194825, 0.6148170, 0.5376921),
        (0.1196985, 0.6184902, 0.5363473),
        (0.1200807, 0.6221608, 0.5349463),
        (0.1206382, 0.6258283, 0.5334883),
        (0.1213797, 0.6294924, 0.5319727),
        (0.1223124, 0.6331527, 0.5303980),
        (0.1234435, 0.6368089, 0.5287634),
        (0.1247795, 0.6404606, 0.5270679),
        (0.1263258, 0.6441074, 0.5253106),
        (0.1280870, 0.6477488, 0.5234909),
        (0.1300668, 0.6513843, 0.5216079),
        (0.1322679, 0.6550136, 0.5196608),
        (0.1346918, 0.6586361, 0.5176488),
        (0.1373392, 0.6622515, 0.5155710),
        (0.1402099, 0.6658592, 0.5134268),
        (0.1433029, 0.6694588, 0.5112154),
        (0.1466164, 0.6730496, 0.5089364),
        (0.1501478, 0.6766313, 0.5065889),
        (0.1538940, 0.6802034, 0.5041721),
        (0.1578514, 0.6837652, 0.5016857),
        (0.1620159, 0.6873163, 0.4991290),
        (0.1663832, 0.6908561, 0.4965016),
        (0.1709484, 0.6943840, 0.4938029),
        (0.1757067, 0.6978996, 0.4910325),
        (0.1806531, 0.7014022, 0.4881893),
        (0.1857826, 0.7048913, 0.4852732),
        (0.1910901, 0.7083663, 0.4822839),
        (0.1965706, 0.7118266, 0.4792210),
        (0.2022190, 0.7152717, 0.4760843),
        (0.2080304, 0.7187009, 0.4728733),
        (0.2140001, 0.7221137, 0.4695877),
        (0.2201238, 0.7255094, 0.4662263),
        (0.2263969, 0.7288875, 0.4627893),
        (0.2328149, 0.7322473, 0.4592767),
        (0.2393739, 0.7355882, 0.4556883),
        (0.2460696, 0.7389097, 0.4520240),
        (0.2528985, 0.7422110, 0.4482835),
        (0.2598567, 0.7454916, 0.4444667),
        (0.2669412, 0.7487508, 0.4405728),
        (0.2741492, 0.7519880, 0.4366009),
        (0.2814768, 0.7552026, 0.4325520),
        (0.2889210, 0.7583939, 0.4284262),
        (0.2964789, 0.7615614, 0.4242234),
        (0.3041479, 0.7647043, 0.4199434),
        (0.3119253, 0.7678220, 0.4155863),
        (0.3198086, 0.7709140, 0.4111521),
        (0.3277958, 0.7739795, 0.4066401),
        (0.3358853, 0.7770179, 0.4020491),
        (0.3440741, 0.7800285, 0.3973810),
        (0.3523598, 0.7830108, 0.3926357),
        (0.3607405, 0.7859641, 0.3878135),
        (0.3692142, 0.7888879, 0.3829143),
        (0.3777789, 0.7917814, 0.3779385),
        (0.3864328, 0.7946441, 0.3728860),
        (0.3951740, 0.7974754, 0.3677572),
        (0.4040010, 0.8002746, 0.3625522),
        (0.4129135, 0.8030409, 0.3572689),
        (0.4219081, 0.8057741, 0.3519100),
        (0.4309831, 0.8084734, 0.3464760),
        (0.4401369, 0.8111383, 0.3409673),
        (0.4493676, 0.8137683, 0.3353842),
        (0.4586736, 0.8163628, 0.3297274),
        (0.4680531, 0.8189214, 0.3239976),
        (0.4775044, 0.8214435, 0.3181952),
        (0.4870258, 0.8239286, 0.3123213),
        (0.4966153, 0.8263763, 0.3063766),
        (0.5062713, 0.8287862, 0.3003621),
        (0.5159918, 0.8311578, 0.2942788),
        (0.5257762, 0.8334906, 0.2881265),
        (0.5356211, 0.8357845, 0.2819083),
        (0.5455244, 0.8380391, 0.2756260),
        (0.5554839, 0.8402543, 0.2692814),
        (0.5654976, 0.8424299, 0.2628768),
        (0.5755629, 0.8445656, 0.2564145),
        (0.5856777, 0.8466613, 0.2498974),
        (0.5958393, 0.8487172, 0.2433287),
        (0.6060452, 0.8507331, 0.2367121),
        (0.6162928, 0.8527091, 0.2300517),
        (0.6265792, 0.8546454, 0.2233525),
        (0.6369015, 0.8565422, 0.2166201),
        (0.6472568, 0.8583999, 0.2098608),
        (0.6576419, 0.8602187, 0.2030822),
        (0.6680536, 0.8619993, 0.1962930),
        (0.6784886, 0.8637421, 0.1895032),
        (0.6889435, 0.8654477, 0.1827245),
        (0.6994146, 0.8671171, 0.1759705),
        (0.7098984, 0.8687509, 0.1692571),
        (0.7203911, 0.8703501, 0.1626027),
        (0.7308890, 0.8719158, 0.1560289),
        (0.7413880, 0.8734491, 0.1495610),
        (0.7518841, 0.8749514, 0.1432282),
        (0.7623734, 0.8764239, 0.1370644),
        (0.7728518, 0.8778680, 0.1311086),
        (0.7833153, 0.8792854, 0.1254053),
        (0.7937599, 0.8806776, 0.1200053),
        (0.8041815, 0.8820463, 0.1149650),
        (0.8145763, 0.8833932, 0.1103467),
        (0.8249402, 0.8847203, 0.1062172),
        (0.8352695, 0.8860294, 0.1026459),
        (0.8455605, 0.8873224, 0.0997021),
        (0.8558096, 0.8886013, 0.0974518),
        (0.8660132, 0.8898681, 0.0959527),
        (0.8761682, 0.8911248, 0.0952504),
        (0.8862714, 0.8923735, 0.0953743),
        (0.8963200, 0.8936161, 0.0963353),
        (0.9063112, 0.8948546, 0.0981249),
        (0.9162421, 0.8960912, 0.1007168),
        (0.9261057, 0.8973297, 0.1040706),
        (0.9359044, 0.8985704, 0.1081309),
        (0.9456362, 0.8998150, 0.1128377),
        (0.9552997, 0.9010653, 0.1181283),
        (0.9648935, 0.9023231, 0.1239405),
        (0.9744166, 0.9035899, 0.1302149),
        (0.9838682, 0.9048672, 0.1368967),
        (0.9932478, 0.9061565, 0.1439362),
    ]
}
