select
    CAST(strength AS float4) as "strength!",
    CAST(dexterity AS float4) as "dexterity!",
    CAST(constitution AS float4) as "constitution!",
    CAST(intelligence AS float4) as "intelligence!",
    CAST(wisdom AS float4) as "wisdom!",
    CAST(charisma AS float4) as "charisma!"
from
    character_attributes