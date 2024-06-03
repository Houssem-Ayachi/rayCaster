#![allow(nonstandard_style)]

use crate::{Vectors::vec2d::Vec2d, Map::S_Map, Square::{SquareType, Square}, Angles::Angle};

#[derive(Debug)]
pub struct Ray{
    pub endPoint: Vec2d,
    pub length: f32
}

impl Ray{

    pub fn rayLengthOnPlane(rayAngleRad: f32, startAngleRad: f32, length: f32) -> f32{
        let rayToPlayerDirAngle = Angle::new(rayAngleRad - startAngleRad);
        return (length * rayToPlayerDirAngle.value.cos()).abs();
    }

    fn castRayY(map: &S_Map ,startPoint: &Vec2d, radAngle: f32, startAngleRad: f32) -> Option<Ray>{
        let mut currentTileY = startPoint.y / map.squareHeight as f32;//get current occupied tile Y in map
        let mut ystep = 0.0;
        //NOTE: for some reason I'm working with a counter clock wise trig unit circle;
        //if angle is going up ystep = -1 else ystep = 1;
        if radAngle >= 0.0 && radAngle <= std::f32::consts::PI{//angle going down
            ystep = 1.0;
            currentTileY += 1.0;
        }else if radAngle >= std::f32::consts::PI{
            ystep = -1.0;
        }

        let mut rayY = currentTileY as i32 * map.squareHeight as i32;//initial ray Y position (TO RETURN LATER)
        let mut dy = rayY as f32 - startPoint.y;// Y offset to next tile
        let mut dx = dy / radAngle.tan();//offset to next X position for the ray
        let mut rayX = startPoint.x + dx;// initial ray X position (TO RETURN LATER)
        let mut rayMapX = rayX as i32 / map.squareWidth as i32;// which square the ray is in on the X axis 
        let mut rayMapY = currentTileY as i32;// which square the ray is in on the Y axis
        
        loop{
            //checking if ray position is inside the map boundaries
            if rayMapX >= 0 && rayMapX < map.cols as i32 && rayMapY >= 0 && rayMapY < map.rows as i32{
                let targetSquare: &Square;
                if ystep < 0.0{
                    targetSquare = map.cells.get((rayMapY + ystep as i32) as usize).unwrap().get(rayMapX as usize).unwrap();
                }else{
                    targetSquare = map.cells.get((rayMapY as i32) as usize).unwrap().get(rayMapX as usize).unwrap();
                }
                if targetSquare.sType == SquareType::WALL {
                    let endPoint = Vec2d::new(rayX, rayY as f32);
                    return Some(Ray{
                        length: Vec2d::difference(startPoint,&endPoint).length(),
                        endPoint,
                    });
                }
                rayMapY += ystep as i32;
                dy = (rayMapY * map.squareHeight as i32) as f32 - rayY as f32;
                rayY = rayY + dy as i32;
                dx = dy / radAngle.tan();
                rayX += dx;
                rayMapX = rayX as i32 / map.squareWidth as i32;
            }else{
                break;
            }
        }
        return None;
    }

    fn castRayX(map: &S_Map ,startPoint: &Vec2d, radAngle: f32, startAngleRad: f32) -> Option<Ray>{
        let mut currentTileX = startPoint.x / map.squareWidth as f32;//get current occupied tile X in map
        let mut xstep:i32 = 0;
        //angle is going left
        if radAngle > std::f32::consts::PI / 2.0 && radAngle <  3.0 * std::f32::consts::PI / 2.0{
            xstep = -1;
        }else{ // angle going right (or straight up or down 'might make it only go right later dunno')
            xstep = 1;
            currentTileX += 1.0;
        }
        let mut rayX = currentTileX as i32 * map.squareWidth as i32;
        let mut dx = rayX - startPoint.x as i32;
        let mut dy = dx as f32 * radAngle.tan();
        let mut rayY = startPoint.y + dy;
        let mut rayMapX = currentTileX as i32;
        let mut rayMapY = rayY as i32 / map.squareHeight as i32;

        loop{
            //checking if ray position is inside the map boundaries
            if rayMapX >= 0 && rayMapX < map.cols as i32 && rayMapY >= 0 && rayMapY < map.rows as i32{
                let targetSquare: &Square;
                if xstep < 0{
                    targetSquare = map.cells.get((rayMapY as i32) as usize).unwrap().get((rayMapX + xstep) as usize).unwrap();
                }else{
                    targetSquare = map.cells.get((rayMapY as i32) as usize).unwrap().get(rayMapX as usize).unwrap();
                }
                if targetSquare.sType == SquareType::WALL {
                    let endPoint = Vec2d::new(rayX as f32, rayY as f32);
                    return Some(Ray{
                        length: Vec2d::difference(startPoint, &endPoint).length(),
                        endPoint,
                    });
                }
                rayMapX += xstep;
                dx = (rayMapX * map.squareWidth as i32) - rayX;
                rayX += dx;
                dy = dx as f32 * radAngle.tan();
                rayY += dy;
                rayMapY = rayY as i32 / map.squareHeight as i32;
            }else{
                return None;
            }
        }
    }

    pub fn castRay(map: &S_Map, startPoint: &Vec2d, angle: f32, startAngleRad: f32) -> Ray{
        let radAngle = angle.to_radians();
        let oRayX = Ray::castRayX(map, startPoint, radAngle, startAngleRad);
        let oRayY = Ray::castRayY(map, startPoint, radAngle, startAngleRad);

        let mut rayX: Ray = match oRayX {
            Some(ray) => ray,
            None => {
                let mut ray = oRayY.unwrap();
                // ray.length = Ray::rayLengthOnPlane(angle, startAngleRad, ray.length);
                return ray;
            },
        };

        let rayY: Ray = match oRayY {
            Some(ray) => ray,
            None => {
                // rayX.length = Ray::rayLengthOnPlane(angle, startAngleRad, rayX.length);
                return rayX;
            }
        };
        let mut finalRay: Ray;
        if rayX.length < rayY.length{
            finalRay = rayX;
        }else{
            finalRay = rayY;
        }
        // finalRay.length = Ray::rayLengthOnPlane(angle, startAngleRad, finalRay.length);
        return finalRay;
    }

    pub fn castRays(map: &S_Map, startPoint: &Vec2d, FOV: f32, numRays: i32, startAngle: &Angle) -> Vec<Ray>{
        let mut rays: Vec<Ray> = Vec::new();
        let start = Angle::new(startAngle.value - (FOV / 2.0));
        let step = (FOV / 2.0) / (numRays / 2) as f32;
        for i in 0..numRays{
            let angle = Angle::new(start.value + (step * i as f32));
            let ray = Ray::castRay(map, startPoint, angle.value, startAngle.value.to_radians());
            rays.push(ray);
        }
        return rays;
    }

    pub fn rayToScreenStartEndPos(&self, screenHeight: i32){
        let lineHeight = (screenHeight as f32 / self.length) as i32;
        
    }
}