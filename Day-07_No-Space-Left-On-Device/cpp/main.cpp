#include <bits/stdc++.h>

struct File
{
    std::string name;
    std::size_t size;
};

class Directory
{
public:
    Directory* parent;
    std::vector<Directory*> subdirs;
    std::vector<File> files;
    std::string name;
    std::string path;

    Directory(const std::string &name=""): name(name)
    {
        path = name + std::string("/");
    }

    void addSubdir(const std::string &name)
    {
        Directory* dir = new Directory(name);
        dir->parent = this;
        dir->path = this->path + dir->name + std::string("/");
        subdirs.push_back(dir);
    }

    void addFile(const File& file)
    {
        files.push_back(file);
    }

    void printFilesystem(std::size_t tabs=0)
    {
        std::cout << std::string(tabs, '\t') << this->name << "/" << std::endl;
        for(auto &i : subdirs)
        {
            i->printFilesystem(tabs+1);
        }
        for(auto &i : files)
        {
            std::cout << std::string(tabs + 1, '\t') << i.name << " " << i.size << std::endl;
        }
    }

    std::size_t getFileSize()
    {
        std::size_t totalSize = 0;
        for(auto &i : subdirs)
        {
            totalSize += i->getFileSize();
        }
        for(auto &i : files)
        {
            totalSize += i.size;
        }
        return totalSize;
    }

    std::size_t getPartOneAnswer()
    {
        std::size_t total = 0;
        for(auto &i : subdirs)
        {
            total += i->getPartOneAnswer();
        }
        std::size_t fsize = getFileSize();
        if(fsize < 100000) total += fsize;
        return total;
    }

    // I know directories are not files but file datatype suits just fine
    std::vector<File> getPartTwoCandidates(std::size_t neededSize)
    {
        std::vector<File> result;
        for(auto &i : subdirs)
        {
            auto b = i->getPartTwoCandidates(neededSize);
            result.insert(result.end(), b.begin(), b.end());
        }
        for(auto &i : subdirs)
        {
            std::size_t fsize = i->getFileSize();
            if(fsize >= neededSize)
            {
                result.push_back({i->name, fsize});
            }
        }
        return result;
    }

    void clearDirs()
    {
        for(auto &i : subdirs)
        {
            i->clearDirs();
            delete i;
        }
    }
};

std::vector<std::string> split(const std::string &str, char delim)
{
    std::string currentStr = std::string();
    std::vector<std::string> result;

    for(std::size_t i = 0; i < str.size(); i++)
    {
        if(str[i] == delim)
        {
            result.push_back(currentStr);
            currentStr = std::string();
        }
        else
        {
            currentStr += str[i];
        }
    }
    result.push_back(currentStr);
    return result;
}

void changeDir(Directory **dirptr, const std::string &name)
{
    if(name == "..")
    {
        if(dirptr != nullptr) *dirptr = (*dirptr)->parent;
        return;
    }
    for(auto &s : (*dirptr)->subdirs)
    {
        if(s->name == name)
        {
            *dirptr = s;
            break;
        }
    }
}

int main()
{
    // std::shared_ptr<Directory> dirPtr = std::make_shared<Directory>("Parent");
    Directory parentDir("");
    Directory *dirPtr = &parentDir;

    std::vector<std::string> inputs;
    while(true)
    {
        std::string input;
        std::getline(std::cin, input);
        if(input.length() == 0) break;
        inputs.push_back(input);
    }

    for(auto &input : inputs)
    {
        auto splitted = split(input, ' ');
        if(splitted[0] == "$")
        {
            if(splitted[1] == "cd")
            {
                if(splitted[2] == "/") continue;
                changeDir(&dirPtr, splitted[2]);
            }
        }
        else if(splitted[0] == "dir")
        {
            dirPtr->addSubdir(splitted[1]);
        }
        else
        {
            std::size_t fsize = std::atoi(splitted[0].c_str());
            std::string fname = splitted[1];
            dirPtr->addFile({fname, fsize});
        }
    }

    std::cout << "Part One: " << parentDir.getPartOneAnswer() << std::endl;

    const std::size_t maxSpace = 70000000;
    std::size_t freeSpace = maxSpace - parentDir.getFileSize();
    const std::size_t reqSpace = 30000000;
    std::size_t neededSpace = reqSpace - freeSpace;

    std::size_t smallestSize = -1;
    for(auto &i : parentDir.getPartTwoCandidates(neededSpace))
    {
        // std::cout << i.name << ": " << i.size << std::endl;
        if(i.size < smallestSize)
        {
            smallestSize = i.size;
        }
    }

    std::cout << "Part Two: " << smallestSize << std::endl;

    parentDir.clearDirs();
    return 0;
}