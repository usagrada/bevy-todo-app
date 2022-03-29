import styled from '@emotion/styled';
import { useCallback, useState, VFC } from 'react';
import { useRecoilState, useRecoilValue } from 'recoil';
import { atom_channels } from '../../stores/tasks';

const Container = styled.div`
    display: grid;
    grid-template-columns: 20vw 80vw;
    min-height: 100vh;
`;

const SidebarWrapper = styled.div`
    background-color: #f5f5f5;
`;

const MainContentWrapper = styled.div`
    padding: 10px;
    background-color: aquamarine;
    overflow: scroll;
`;

const ChannelList = styled.div`
    padding: 10px;
    background-color: aquamarine;
    &:hover {
        background-color: #7070f1;
    }
`;

const Sidebar: VFC = () => {
    const [channels, setChannels] = useRecoilState(atom_channels);
    const selectChannel = useCallback(
        (name: string) => {
            setChannels({ ...channels, select: name });
        },
        [channels.channels],
    );
    return (
        <SidebarWrapper>
            <div style={{ padding: '10px' }}>Sidebar</div>
            {channels.channels.map((channel) => {
                let onclick = () => {
                    selectChannel(channel.name);
                };
                return (
                    <ChannelList key={channel.name} onClick={onclick}>
                        # {channel.name}
                    </ChannelList>
                );
            })}
        </SidebarWrapper>
    );
};

interface Tasks {
    lists: string[];
    tasks: Task[];
}

interface Task {
    name: string;
    list: string;
}

const ListWrapper = styled.div`
    display: flex;
    margin: 10px;
`;

const ListComponent = styled.div`
    flex-shrink: 0;
    padding: 10px;
    margin: 10px;
    width: 30%;
    background-color: #f5f5f5;
    border-radius: 5px;
`;

const ModalWindow = styled.div`
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    padding: 10vh 10vw;
    background: rgba(21, 21, 21, 0.4);
`;

const ModalContent = styled.div`
    width: 80vw;
    height: 80vh;
    padding: 20px;
    border-radius: 10px;
    background-color: #f5f5f5;
`;

const MainContent: VFC = () => {
    const channels = useRecoilValue(atom_channels);
    const tasks: Tasks = {
        lists: ['list1', 'list2', 'list3', 'list4', 'list5'],
        tasks: [
            { name: 'task1', list: 'list1' },
            { name: 'task2', list: 'list1' },
            { name: 'task3', list: 'list2' },
            { name: 'task4', list: 'list2' },
        ],
    };
    const [toggle, setToggle] = useState(false);
    const modalOpen = () => setToggle(true);

    return (
        <MainContentWrapper>
            <div style={{ padding: '10px' }}># {channels.select}</div>
            {toggle && (
                <ModalWindow
                    onClick={() => {
                        setToggle(false);
                    }}
                >
                    <ModalContent
                        onClick={(e) => {
                            e.stopPropagation();
                        }}
                    >
                        modal content
                    </ModalContent>
                </ModalWindow>
            )}
            <ListWrapper>
                {tasks.lists.map((list) => {
                    return (
                        <ListComponent>
                            <div>{list}</div>
                            <div>
                                {tasks.tasks
                                    .filter((task) => task.list == list)
                                    .map((task) => {
                                        return <div onClick={modalOpen}>{task.name}</div>;
                                    })}
                            </div>
                        </ListComponent>
                    );
                })}
            </ListWrapper>
        </MainContentWrapper>
    );
};

const TaskPage: VFC = () => {
    return (
        <Container>
            <Sidebar />
            <MainContent />
        </Container>
    );
};

export default TaskPage;
