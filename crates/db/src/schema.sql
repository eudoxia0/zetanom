create table foods (
    food_id integer primary key,
    -- Name of this food.
    name text not null,
    -- Name of the brand. `null` for generic foods like fruits.
    brand text,
    -- One of `g` or `ml`. A "serving" is 100 * serving_unit.
    serving_unit text not null,

    -- Energy per serving in kcal.
    energy real not null,
    -- Protein per serving in g.
    protein real not null,
    -- Fat per serving in g.
    fat real not null,
    -- Saturated fat per serving in g.
    fat_saturated real not null,
    -- Available carbohydrate (excluding fiber) per serving in g.
    carbs real not null,
    -- Sugars per serving in g.
    carbs_sugars real not null,
    -- Dietary fibre per serving in g.
    fibre real not null,
    -- Sodium per serving in mg.
    sodium real not null,

    -- Timestamp when this record was created.
    created_at text not null,

    -- Constraint: allowed values for `serving_unit`.
    check(serving_unit in ('g', 'ml'))
) strict;

create table serving_sizes (
    serving_id integer primary key,
    food_id integer not null,
    -- Name of this serving, e.g., "bottle", "package", "cup", "slice".
    serving_name text not null,
    -- Amount in the food's `serving_unit` (g or ml).
    serving_amount real not null,

    foreign key (food_id) references foods(food_id) on delete cascade,
    unique(food_id, serving_name)
) strict;
