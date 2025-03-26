import { invoke } from "@tauri-apps/api/core";
import { Line } from 'react-chartjs-2';
import { useEffect, useState, useRef } from 'react';
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
} from 'chart.js';

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

const options = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
        title: {
            display: true,
            text: 'Historial de mantenimiento',
            font: {
                size: 20,
            },
        },
        legend: {
            labels: {
                color: 'rgb(255, 99, 132)',
            },
            onClick: () => {},
        },
    },
    scales: {
        x: {
            grid: {},
            ticks: {
                color: 'rgb(255, 99, 132)',
            },
        },
        y: {
            grid: {},
            ticks: {
                color: 'rgb(255, 99, 132)',
            },
        },
    },
};

const labels = ['Enero', 'Febrero', 'Marzo', 'Abril', 'Mayo', 'Junio', 'Julio', 'Agosto', 'Septiembre', 'Octubre', 'Noviembre', 'Diciembre'];

const initialData = {
    labels,
    datasets: [
        {
            label: 'Progreso',
            data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            pointBorderWidth: 4,
            borderWidth: 2,
            borderColor: 'rgb(255, 130, 99)',
            backgroundColor: 'rgb(218, 78, 44)',
        }
    ],
};

export default function LineGraph() {
    const [chartData, setChartData] = useState(initialData);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState(null);
    const dataFetchedRef = useRef(false);

    useEffect(() => {
        // Evitamos la doble llamada usando un ref para rastrear si ya se ha llamado
        if (dataFetchedRef.current) return;
        
        const fetchMaintenanceStats = async () => {
            try {
                console.log("Llamando a get_maintenance_stats...");
                dataFetchedRef.current = true;
                setIsLoading(true);
                
                const response = await invoke('get_maintenance_stats');
                
                const statsData = typeof response === 'string' ? JSON.parse(response) : response;
                
                console.log('Datos de mantenimiento recibidos:', statsData);
                
                // Basado en la estructura actual: {"enero":0,"febrero":15,"marzo":20,"abril":15,...}
                if (statsData) {
                    const monthKeys = [
                        'enero', 'febrero', 'marzo', 'abril', 'mayo', 'junio',
                        'julio', 'agosto', 'septiembre', 'octubre', 'noviembre', 'diciembre'
                    ];
                    
                    const monthlyData = monthKeys.map(month => 
                        statsData[month] !== undefined ? statsData[month] : 0
                    );
                    
                    setChartData({
                        labels,
                        datasets: [
                            {
                                ...initialData.datasets[0],
                                data: monthlyData
                            }
                        ]
                    });
                }
                
                setIsLoading(false);
            } catch (err) {
                console.error('Error al obtener estadísticas de mantenimiento:', err);
                setError(err.toString());
                setIsLoading(false);
            }
        };
        
        fetchMaintenanceStats();
        
    }, []);
    
    if (isLoading) {
        return <div>Cargando datos de mantenimiento...</div>;
    }
    
    if (error) {
        return <div>Error al cargar datos: {error}</div>;
    }
    
    return <Line options={options} data={chartData} />;
}