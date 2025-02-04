from .bool_pack import BoolPack, OutfitType, Category

import orjson


def gen_outfit(clothes: str, bool_pack: int) -> str:

    clothes: list[str, int | str | bool] = orjson.loads(clothes)

    pack = BoolPack.from_int(bool_pack)

    colors = pack.get_colors()

    filtered_clothes = [clothe for clothe in clothes if clothe["color"] in colors]

    outfit_type = pack.get_outfit_type()

    selected_clothes = []

    categories = set()

    for clothe in filtered_clothes:
        clothe_category = Category(clothe["category"])

        if clothe_category in categories:
            continue
        if clothe_category in {
            Category.Dress,
            Category.Shorts,
            Category.Pants,
            Category.Skirt,
        } and any(
            c in categories
            for c in {Category.Dress, Category.Shorts, Category.Pants, Category.Skirt}
        ):
            continue

        if outfit_type == OutfitType.LightAndBreezy and clothe["is_for_hot_weather"]:
            selected_clothes.append(clothe)
            categories.add(clothe_category)
        elif (
            outfit_type == OutfitType.HeavyAndInsulated
            and not clothe["is_for_hot_weather"]
        ):
            selected_clothes.append(clothe)
            categories.add(clothe_category)
        elif outfit_type in [
            OutfitType.ComfortableAndCasual,
            OutfitType.ABitLayered,
            OutfitType.CozyAndWarm,
        ]:
            selected_clothes.append(clothe)
            categories.add(clothe_category)

    return orjson.dumps(selected_clothes).decode()


if __name__ == "__main__":
    gen_outfit(
        """
    [{"name":"rosinha","color":"Pink","category":"Dress","is_hot_weather":false},{"name":"calções pijama","color":"Gray","category":"Shorts","is_hot_weather":false},{"name":"bestiudo trump","color":"Orange","category":"Dress","is_hot_weather":true},{"name":"DOURO","color":"Gold","category":"Shorts","is_hot_weather":true}]
""",
        13312,
    )
