import React, { useEffect, useRef, useState } from 'react'; import { Play, Pause, RotateCcw } from 'lucide-react';

const EarthGlobe = () => { const canvasRef = useRef(null); const [isRotating, setIsRotating] = useState(true); const [rotationSpeed, setRotationSpeed] = useState(0.01); const animationRef = useRef(null); const rotationRef = useRef(0);

useEffect(() => { const canvas = canvasRef.current; if (!canvas) return;

const ctx = canvas.getContext('2d');
const width = canvas.width;
const height = canvas.height;
const centerX = width / 2;
const centerY = height / 2;
const radius = 150;

const continents = [
  { lat: 0, lon: 20, points: [[0,20], [10,25], [15,35], [5,40], [-10,35], [-15,25], [-5,20]] },
  { lat: 50, lon: 50, points: [[40,30], [60,40], [80,45], [100,40], [90,55], [70,60], [50,65], [30,55]] },
  { lat: 45, lon: -100, points: [[35,-120], [50,-110], [60,-95], [55,-85], [40,-90], [30,-100]] },
  { lat: -15, lon: -60, points: [[-5,-70], [5,-65], [10,-55], [0,-50], [-10,-55], [-15,-65]] },
  { lat: -25, lon: 135, points: [[-20,125], [-15,135], [-25,145], [-35,140], [-30,130]] }
];

const projectPoint = (lat, lon, rotation) => {
  const latRad = (lat * Math.PI) / 180;
  const lonRad = ((lon + rotation) * Math.PI) / 180;
  const x = radius * Math.cos(latRad) * Math.sin(lonRad);
  const y = radius * Math.sin(latRad);
  const z = radius * Math.cos(latRad) * Math.cos(lonRad);
  return { x: centerX + x, y: centerY - y, z };
};

const drawGlobe = (rotation) => {
  ctx.clearRect(0, 0, width, height);

  const gradient = ctx.createRadialGradient(
    centerX - radius * 0.3, 
    centerY - radius * 0.3, 
    radius * 0.1,
    centerX, 
    centerY, 
    radius
  );
  gradient.addColorStop(0, '#4fc3f7');
  gradient.addColorStop(0.5, '#2196f3');
  gradient.addColorStop(1, '#0d47a1');
  
  ctx.beginPath();
  ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
  ctx.fillStyle = gradient;
  ctx.fill();

  ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
  ctx.lineWidth = 1;
  for (let lat = -60; lat <= 60; lat += 30) {
    ctx.beginPath();
    for (let lon = -180; lon <= 180; lon += 5) {
      const point = projectPoint(lat, lon, rotation);
      if (point.z > 0) {
        if (lon === -180) ctx.moveTo(point.x, point.y);
        else ctx.lineTo(point.x, point.y);
      }
    }
    ctx.stroke();
  }

  for (let lon = -180; lon <= 180; lon += 30) {
    ctx.beginPath();
    for (let lat = -90; lat <= 90; lat += 5) {
      const point = projectPoint(lat, lon, rotation);
      if (point.z > 0) {
        if (lat === -90) ctx.moveTo(point.x, point.y);
        else ctx.lineTo(point.x, point.y);
      }
    }
    ctx.stroke();
  }

  ctx.fillStyle = '#4caf50';
  ctx.strokeStyle = '#2e7d32';
  ctx.lineWidth = 2;

  continents.forEach(continent => {
    continent.points.forEach((point, i) => {
      const projectedPoint = projectPoint(point[0], point[1], rotation);
      if (projectedPoint.z > 0) {
        if (i === 0) {
          ctx.beginPath();
          ctx.moveTo(projectedPoint.x, projectedPoint.y);
        } else ctx.lineTo(projectedPoint.x, projectedPoint.y);
      }
    });
    ctx.closePath();
    ctx.fill();
    ctx.stroke();
  });

  const highlightGradient = ctx.createRadialGradient(
    centerX - radius * 0.4,
    centerY - radius * 0.4,
    0,
    centerX - radius * 0.4,
    centerY - radius * 0.4,
    radius * 0.5
  );
  highlightGradient.addColorStop(0, 'rgba(255, 255, 255, 0.4)');
  highlightGradient.addColorStop(1, 'rgba(255, 255, 255, 0)');

  ctx.beginPath();
  ctx.arc(centerX, centerY, radius, 0, Math.PI * 2);
  ctx.fillStyle = highlightGradient;
  ctx.fill();
};

const animate = () => {
  if (isRotating) rotationRef.current += rotationSpeed * 100;
  drawGlobe(rotationRef.current);
  animationRef.current = requestAnimationFrame(animate);
};

animate();

return () => {
  if (animationRef.current) cancelAnimationFrame(animationRef.current);
};

}, [isRotating, rotationSpeed]);

const handleReset = () => { rotationRef.current = 0; };

return ( <div className="flex flex-col items-center justify-center min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 p-8"> <div className="bg-slate-800/50 backdrop-blur-sm rounded-2xl p-8 shadow-2xl border border-slate-700"> <h1 className="text-3xl font-bold text-white mb-6 text-center">пошел нахуй</h1> <canvas ref={canvasRef} width={600} height={600} className="rounded-xl shadow-2xl mb-6 bg-black/20" /> <div className="flex flex-col gap-4"> <div className="flex gap-3 justify-center"> <button onClick={() => setIsRotating(!isRotating)} className="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors shadow-lg"> {isRotating ? <Pause size={20} /> : <Play size={20} />} {isRotating ? 'Пауза' : 'Запустить'} </button> <button onClick={handleReset} className="flex items-center gap-2 px-6 py-3 bg-slate-600 hover:bg-slate-700 text-white rounded-lg transition-colors shadow-lg"> <RotateCcw size={20} /> Сброс </button> </div> <div className="bg-slate-700/50 rounded-lg p-4"> <label className="text-white text-sm font-medium mb-2 block">Скорость вращения: {rotationSpeed.toFixed(3)}</label> <input type="range" min="0.001" max="0.05" step="0.001" value={rotationSpeed} onChange={(e) => setRotationSpeed(parseFloat(e.target.value))} className="w-full h-2 bg-slate-600 rounded-lg appearance-none cursor-pointer" /> </div> </div> <div className="mt-6 text-center text-slate-300 text-sm"> <p>бебебе</p> <p className="text-slate-400 mt-1">React :з</p> </div> </div> </div> ); };

export default EarthGlobe;