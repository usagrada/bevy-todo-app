import { channel } from 'diagnostics_channel';
import { atom } from 'recoil';

interface Channels {
    channels: Channel[];
    select: string;
}

interface Channel {
    name: string;
    lists: string[];
}

export const atom_channels = atom<Channels>({
    key: 'channels',
    default: {
        channels: [
            { name: 'general', lists: ['list1', 'list2', 'list3', 'list4', 'list5'] },
            { name: 'random', lists: ['TODO', 'DONE'] },
        ],
        select: '',
    },
});

interface Tasks {
    lists: string[];
    tasks: Task[];
}

interface Task {
    name: string;
    list: string;
}

export const atom_tasks = atom<Tasks>({
    key: 'tasks',
    default: {
        lists: ['list1', 'list2', 'list3', 'list4', 'list5'],
        tasks: [
            { name: 'task1', list: 'list1' },
            { name: 'task2', list: 'list1' },
            { name: 'task3', list: 'list2' },
            { name: 'task4', list: 'list2' },
        ],
    },
});
